mod event;
mod request;
mod state;

use crate::elevator::{event::Event, request::Requests, state::State};
use crate::error::ElevatorError;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

pub struct Elevator {
    floor: usize,
    n_floors: usize,
    requests: Requests,
    state: State,
    direction: Option<Direction>,
}


impl Elevator {
    pub fn new(floor: usize, n_floors: usize) -> Elevator {
        let requests = Requests::new(n_floors);
        let state = State::Idle;
        let direction = None;
        Elevator {floor, n_floors, requests, state, direction}
    }

    /*
    pub fn change_floor(&mut self) -> Result<(), ElevatorError> { 
        let direction = self.direction.ok_or(None)?;

        match direction {
            Direction::Up => {
                if self.floor < self.n_floors - 2 {
                    self.floor += 1;
                } else {
                    Err(Some(self.floor))
                }
            }
            Direction::Down => {
                if self.floor > 0 {
                    self.floor -= 1;
                } else {
                    Err(Some(self.floor))
                }
            }
        }

        Ok(())
    }

    pub fn check_for_request(&self, floor: usize) -> bool {
        self.requests.internal[floor] || self.requests.external[floor]
    }
    */

    pub fn state_machine(_thread_id: usize, n_floors: usize) -> Result<(), ElevatorError> {
        let event = Event::TimerTimedOut;
        let _elevator = Elevator::new(0, n_floors);
    
        loop { //replace loop with wait for event
            match event {
                Event::ButtonPress(_floor) => {
                    //add request for floor, and optionally change state
                }
                Event::ArriveAtFloor(_floor) => {
                    /*
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
                    */
                }
                Event::TimerTimedOut => {
                    // what to do
                    //state.move(Direction::Up);
                    //event = Event::ArriveAtFloor(1);
                }
            }
    
            break;
        }
    
        Ok(())
    }
}


