#![allow(dead_code)]
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

mod elevator;
mod error;
mod network;
mod state_machine;

use crate::error::ElevatorError;

pub struct Config {
    pub n_elevators: usize,
    pub n_floors: usize,
}

pub enum Message {
    Request { floor: usize },
    Shutdown,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Executing run function");

    let Config { n_elevators, n_floors} = config;
    println!("Number of elevator: {n_elevators}");
    println!("Number of floors: {n_floors}");

    let mut handles: Vec<JoinHandle<_>> = Vec::with_capacity(n_elevators);
    let mut transmitters: Vec<Sender<Message>> = Vec::with_capacity(n_elevators);

    const HOST: [u8; 4] = [127, 0, 0, 1];
    const BASE_PORT: u16 = 10000;

    for i in 0..n_elevators {
        let addr = SocketAddr::from((HOST, BASE_PORT + i as u16));
        println!("Thread {i} attempting to connect to {addr}");
        let stream = TcpStream::connect(addr)?;
        println!("Thread {i} connected to {addr}");
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || -> Result<(), ElevatorError> {
            state_machine::run(stream, rx, n_floors)
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

        // DISTRIBUTE REQUESTS
        /*
        // TODO
        */
    }

    Ok(())
}
