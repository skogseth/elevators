pub mod event;
pub mod request;
pub mod state;

use crate::elevator::request::Requests;
use crate::elevator::state::{State, direction::Direction};
use crate::error::ElevatorError;

pub struct Elevator {
    pub floor: usize,
    n_floors: usize,
    pub requests: Requests,
    pub state: State,
}


impl Elevator {
    pub fn new(floor: usize, n_floors: usize) -> Elevator {
        let requests = Requests::new(n_floors);
        let state = State::Idle;
        Elevator {floor, n_floors, requests, state}
    }

    pub fn error(&self, critical: bool) -> ElevatorError {
        ElevatorError {
            floor: self.floor,
            state: self.state,
            critical
        }
    }

    pub fn get_n_floors(&self) -> usize {self.n_floors }
    //pub fn get_state(&self) -> State { self.state }
    //pub fn set_state(&mut self, state: State) { self.state = state }
}

impl std::fmt::Display for Elevator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s1 = format!("elevator on floor {} in state {}", self.floor, self.state);
        let s2 = match self.state {
            State::Idle => String::new(),
            State::Moving(dir) | State::Still(dir) => format!("with direction {}", dir),
        };
        write!(f, "{} {}", s1, s2)
    }
}