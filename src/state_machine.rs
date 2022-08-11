use std::net::TcpStream;
use std::time::Instant;

use crate::elevator::Elevator;
use crate::elevator::event::{Event, button::Button};
use crate::elevator::{/*direction::Direction,*/ /*request::Requests,*/ state::State};
use crate::error::ElevatorError;
use crate::network::{get, send};

const TIMEOUT: u128 = 10;

pub fn run(stream: TcpStream, n_floors: usize) -> Result<(), ElevatorError> {
    let mut elevator = Elevator::new(0, n_floors);

    loop {
        let events = wait_for_event(stream, n_floors);
        
        for event in events {
            match event {
                Event::ArriveAtFloor(floor) => {
                    if let State::Moving(dir) = elevator.state {
                        elevator.floor = floor;
                        
                        //check for request => stop and wait
                        /*
                        if elevator.check_for_request(floor) {
                            elevator.set_state(State::Still);
                        }
                        */
                    } else {
                        eprintln!("arrived at floor {:?} without moving, state: {:?}", floor, elevator.state);
                        // TODO? return Err(elevator.error(true));
                    }
                }
                Event::ButtonPress(_button, _floor) => {
                    //add request for floor, and optionally change state
                }
                Event::TimerTimedOut => {
                    // what to do
                    //elevator.move(Direction::Up);
                    //event = Event::ArriveAtFloor(1);
                }
            }
        }

        break; // TODO: Remove this
    }

    Ok(())
}

fn wait_for_event(mut stream: TcpStream, n_floors: usize) -> Vec<Event> {
    let now = Instant::now();
    let mut events = Vec::new();
    loop {
        // CHECK FOR FLOOR ARRIVE EVENT
        match get::floor(&mut stream) {
            Ok(opt_floor) => if let Some(floor) = opt_floor {
                events.push(Event::ArriveAtFloor(floor));
            }
            Err(_e) => {
                eprintln!("caught error in get::floor()!");
                // TODO
            }
        }
    
        // CHECK FOR BUTTON PRESS
        for floor in 0..n_floors {
            for button in Button::iterator() {
                if let Ok(_pressed) = get::order_button(&mut stream, button, floor) {
                    events.push(Event::ButtonPress(button, floor));
                } else {
                    eprintln!("caught error in get::order_button() for floor {floor:?} & button {button:?}");
                    // TODO
                }
            }
        }

        if events.len() > 0 {
            break;
        } else if now.elapsed().as_micros() > TIMEOUT {
            return vec![Event::TimerTimedOut];
        }
    }
    events
}