#![allow(dead_code)]

use std::io;
use std::thread;

mod elevator;
mod request;
mod state_machine;

pub struct Config {
    pub n_elevators: usize,
    pub n_floors: usize,
}

pub fn run(config: Config) -> Result<(), io::Error> {
    println!("Executing run function");

    let n = config.n_elevators;
    let m = config.n_floors;

    println!("(n, m) = ({n}, {m})");

    let mut handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(n);

    for i in 0..n {
        let handle = thread::spawn(move || -> Result<(), io::Error> {
            state_machine::go(i)
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().expect("Thread panicked, caught by main!");
        if let Err(e) = result {
            return Err(e);
        }
    }

    Ok(())
}