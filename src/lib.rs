#![allow(dead_code)]

mod error;
pub mod elevator;
pub mod network;
mod state_machine;

use crate::error::ElevatorError;

use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::thread;

pub struct Config {
    pub n_elevators: usize,
    pub n_floors: usize,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Executing run function");

    let n = config.n_elevators;
    let m = config.n_floors;

    println!("(n, m) = ({n}, {m})");

    let mut handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(n);

    const HOST: [u8; 4] = [127, 0, 0, 1];
    const BASE_PORT: u16 = 10000;

    for i in 0..n {
        let addr = SocketAddr::from((HOST, BASE_PORT + i as u16));
        let stream = TcpStream::connect(addr)?;
        let handle = thread::spawn(move || -> Result<(), ElevatorError> {
            state_machine::run(stream, n)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked, caught by main!")?;
    }

    Ok(())
}