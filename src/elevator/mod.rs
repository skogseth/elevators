pub mod button;
pub mod direction;
pub mod event;
pub mod request;
pub mod state;

use crate::elevator::{direction::Direction, request::Requests, state::State};
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

    pub fn get_state(&self) -> State { self.state }
    pub fn set_state(&mut self, state: State) { self.state = state }
}

impl std::fmt::Display for Elevator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.direction {
            Some(direction) => write!(f, "elevator on floor {} in state {} with direction {}", self.floor, self.state, direction),
            None => write!(f, "elevator on floor {} in state {} with no direction", self.floor, self.state),
        }
    }
}