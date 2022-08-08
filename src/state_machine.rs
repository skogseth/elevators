use std::net::TcpStream;
use std::time::Instant;

use crate::elevator::Elevator;
use crate::elevator::{/*direction::Direction,*/ event::Event, /*request::Requests,*/ state::State};
use crate::error::ElevatorError;
use crate::network::{get, send};

const TIMEOUT: u128 = 10;

pub fn run(stream: TcpStream, n_floors: usize) -> Result<(), ElevatorError> {
    let mut stream = stream;
    let mut elevator = Elevator::new(0, n_floors);

    loop {
        // Wait for event
        let now = Instant::now();
        let event = loop {
            // CHECK FOR BUTTON PRESS


            // CHECK FOR FLOOR ARRIVE EVENT
            match get::floor(&mut stream) {
                Ok(opt_floor) => if let Some(floor) = opt_floor {
                    break Event::ArriveAtFloor(floor);
                }
                Err(_e) => {
                    // TODO
                }
            }

            // CHECK IF TIMER IS OUT
            if now.elapsed().as_micros() > TIMEOUT {
                break Event::TimerTimedOut;
            }
        };
        
        // Handle event
        match event {
            Event::ButtonPress(_floor) => {
                //add request for floor, and optionally change state
            }
            Event::ArriveAtFloor(floor) => {
                if elevator.get_state() == State::Moving {
                    elevator.change_floor()?;
                    
                    if elevator.check_for_request(floor) {
                        elevator.set_state(State::Still);
                    }
                } else {
                    return Err(elevator.error(true));
                }
            }
            Event::TimerTimedOut => {
                // what to do
                //elevator.move(Direction::Up);
                //event = Event::ArriveAtFloor(1);
            }
        }

        break;
    }

    Ok(())
}