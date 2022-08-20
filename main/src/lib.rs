#![allow(dead_code)]
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

mod error;
mod state_machine;
mod types;

use crate::error::ElevatorError;
use crate::types::{Message, ThreadInfo};

pub struct Config {
    pub n_elevators: usize,
    pub n_floors: usize,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Executing run function");

    let Config {
        n_elevators,
        n_floors,
    } = config;
    println!("Number of elevator: {n_elevators}");
    println!("Number of floors: {n_floors}");

    let mut threads = Vec::new();
    let (tx_thread, rx) = mpsc::channel();

    const HOST: [u8; 4] = [127, 0, 0, 1];
    const BASE_PORT: u16 = 10000;

    for i in 0..n_elevators {
        let addr = SocketAddr::from((HOST, BASE_PORT + i as u16));
        let stream = TcpStream::connect(addr)?;
        println!("Thread {i} connected to {addr}");
        let (tx, rx_thread) = mpsc::channel();
        let tx_thread = tx_thread.clone();

        let handle = thread::spawn(move || -> Result<(), ElevatorError> {
            state_machine::run(i, stream, (tx_thread, rx_thread), n_floors)
        });

        threads.push(ThreadInfo::new(i, handle, tx));
    }

    loop {
        // CHECK FOR ELEVATOR CRASHES
        for i in 0..threads.len() {
            if threads[i].handle.is_finished() {
                let thread = threads.remove(i);
                thread
                    .handle
                    .join()
                    .expect("Thread panicked, caught by main!")
                    .expect("Elevator crashed");
            }
        }

        // CHECK IF ALL ELEVATORS ARE DOWN
        if threads.is_empty() {
            break;
        }

        // TALK WITH ELEVATOR THREADS
        match rx.try_recv() {
            Ok(msg) => match msg {
                Message::Request { floor, direction } => {
                    let mut tx = &threads[0].transmitter;
                    let mut min_cost = std::usize::MAX;
                    for thread in threads.iter() {
                        let cost = thread.cost_function(floor, direction);
                        if cost < min_cost {
                            tx = &thread.transmitter;
                            min_cost = cost;
                        }
                    }
                    tx.send(msg).unwrap();
                }
                Message::HallButtonLight { .. } => {
                    // Send message to all elevators for hall button light
                    for thread in threads.iter() {
                        thread.transmitter.send(msg).unwrap();
                    }
                }
                Message::ElevatorInfo {
                    thread_id,
                    floor,
                    state,
                    n_requests,
                } => {
                    for thread in threads.iter_mut() {
                        if thread.id == thread_id {
                            thread.floor = floor;
                            thread.state = state;
                            thread.n_requests = n_requests;
                            break;
                        }
                    }
                }
                Message::Shutdown => {
                    eprint!("Received shutdown message from thread");
                }
            },
            Err(e) => {
                if e == TryRecvError::Disconnected {
                    break;
                }
            }
        }
    }

    Ok(())
}
