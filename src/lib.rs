#![allow(dead_code)]

mod error;
mod elevator;

use crate::error::ElevatorError;
use crate::elevator::{Elevator, event::Event};
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
            state_machine(i, n)
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked, caught by main!")?;
    }

    Ok(())
}

pub fn state_machine(_thread_id: usize, n_floors: usize) -> Result<(), ElevatorError> {
    let _event = Event::TimerTimedOut;
    let _elevator = Elevator::new(0, n_floors);

    /*
    loop { //replace loop with wait for event
        match event {
            Event::ButtonPress(_floor) => {
                //add request for floor, and optionally change state
            }
            Event::ArriveAtFloor(floor) => {
                if elevator.state == State::Moving {
                    if let Err(e) = elevator.change_floor() {
                        return Err(io::Error::new(io::ErrorKind::Other, "Tried to change floor with no direction at new floor without moving"));
                    }
                    if elevator.check_for_request(floor) {
                        elevator.state = State::Still;
                    }
                } else {
                    return Err(io::Error::new(io::ErrorKind::Other, "Arrived at new floor without moving"));
                }
            }
            Event::TimerTimedOut => {
                state.move(Direction::Up);
                event = Event::ArriveAtFloor(1);
            }
        }

        break;
    }
    */

    Ok(())
}