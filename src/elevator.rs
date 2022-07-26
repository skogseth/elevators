pub mod event;
pub mod request;
pub mod state;

use crate::elevator::{request::Requests, state::State};
//use crate::error::ElevatorError;

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
}
