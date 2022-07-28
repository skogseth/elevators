pub mod direction;
pub mod event;
pub mod request;
pub mod state;

use crate::elevator::{direction::Direction, event::Event, request::Requests, state::State};
use crate::error::ElevatorError;

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

    pub fn error(&self, critical: bool) -> ElevatorError {
        ElevatorError {
            floor: self.floor,
            state: self.state,
            direction: self.direction,
            critical
        }
    }

    pub fn change_floor(&mut self) -> Result<(), ElevatorError> { 
        let direction = self.direction.ok_or(self.error(false))?;

        match direction {
            Direction::Up => {
                if self.floor < self.n_floors - 2 {
                    self.floor += 1;
                } else {
                    return Err(self.error(false));
                }
            }
            Direction::Down => {
                if self.floor > 0 {
                    self.floor -= 1;
                } else {
                    return Err(self.error(false));
                }
            }
        }

        Ok(())
    }

    pub fn check_for_request(&self, floor: usize) -> bool {
        self.requests.internal[floor] || self.requests.external[floor]
    }
}

impl std::fmt::Display for Elevator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.direction {
            Some(direction) => write!(f, "elevator on floor {} in state {} with direction {}", self.floor, self.state, direction),
            None => write!(f, "elevator on floor {} in state {} with no direction", self.floor, self.state),
        }
    }
}

pub fn state_machine(_thread_id: usize, n_floors: usize) -> Result<(), ElevatorError> {
    let mut event = Event::TimerTimedOut;
    let mut elevator = Elevator::new(0, n_floors);

    loop { //replace loop with wait for event
        match event {
            Event::ButtonPress(_floor) => {
                //add request for floor, and optionally change state
            }
            Event::ArriveAtFloor(floor) => {
                if elevator.state == State::Moving {
                    elevator.change_floor()?;
                    
                    if elevator.check_for_request(floor) {
                        elevator.state = State::Still;
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
