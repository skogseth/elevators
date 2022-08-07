use std::net::TcpStream;

use crate::elevator::Elevator;
use crate::elevator::{direction::Direction, event::Event, request::Requests, state::State};
use crate::error::ElevatorError;

pub fn run(stream: TcpStream, n_floors: usize) -> Result<(), ElevatorError> {
    //let event = Event::TimerTimedOut;
    let mut elevator = Elevator::new(0, n_floors);

    loop {
        let event = wait_for_event(stream);
        
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

pub fn wait_for_event(stream: TcpStream) -> Event {
    Event::TimerTimedOut
}