#![allow(dead_code)]

mod error;
mod elevator;

use crate::error::ElevatorError;
use crate::elevator::Elevator;

use std::error::Error;
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

    for i in 0..n {
        let handle = thread::spawn(move || -> Result<(), ElevatorError> {
            Elevator::state_machine(i, n)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked, caught by main!")?;
    }

    Ok(())
}