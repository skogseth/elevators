#![allow(dead_code)]

use std::error::Error;

mod elevator;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Executing run function");

    let n = config.n_elevators;
    let m = config.n_floors;

    println!("(n, m) = ({n}, {m})");

    for i in 1..(n+1) {
        thread::spawn(|| {
            run_elevator(i);
        });
    }

    Ok(())
}

pub struct Config {
    pub n_elevators: usize,
    pub n_floors: usize,
}

enum Direction {
    Up,
    Down,
}

struct Elevator {
    floor: usize,
    dir: Direction,
    requests: Vec<usize>,
}