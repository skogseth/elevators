#![allow(dead_code)]
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;

mod elevator;
mod error;
mod network;
mod state_machine;

use crate::elevator::state::direction::Direction;
use crate::error::ElevatorError;

pub struct Config {
    pub n_elevators: usize,
    pub n_floors: usize,
}

#[derive(Debug)]
pub enum Message {
    Request { floor: usize, direction: Direction },
    Shutdown,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Executing run function");

    let Config {
        n_elevators,
        n_floors,
    } = config;
    println!("Number of elevator: {n_elevators}");
    println!("Number of floors: {n_floors}");

    let mut handles = Vec::with_capacity(n_elevators);
    let mut transmitters = Vec::with_capacity(n_elevators);
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
            state_machine::run(stream, (tx_thread, rx_thread), n_floors)
        });
        handles.push(handle);
        transmitters.push(tx);
    }

    loop {
        // CHECK FOR ELEVATOR CRASHES
        for i in 0..handles.len() {
            if handles[i].is_finished() {
                handles
                    .remove(i)
                    .join()
                    .expect("Thread panicked, caught by main!")
                    .expect("Elevator crashed");
                transmitters.remove(i);
            }
        }

        // CHECK IF ALL ELEVATORS ARE DOWN
        if handles.is_empty() {
            break;
        }

        match rx.try_recv() {
            Ok(msg) => {
                match msg {
                    Message::Request { floor: _, direction: _ } => {
                        // TODO: Implement algorithm to find best elevator
                        let i = 0;
                        transmitters[i].send(msg).unwrap();
                    }
                    Message::Shutdown => {
                        // TODO: Remove 
                    }
                }
            }
            Err(e) => {
                if e == TryRecvError::Disconnected {
                    break;
                }
            }
        }
    }

    Ok(())
}
