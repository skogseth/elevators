use std::collections::HashMap;
use std::time::{Duration, Instant};

use interface::types::{Button, Floor};

use crate::state_machine::types::State;

pub mod requests;
pub mod timer;

use self::requests::Array;
use super::Elevator;

pub struct Requests {
    map: HashMap<Button, Array<bool>>,
    active_buttons: HashMap<Button, Array<bool>>,
    n_floors: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Timer {
    now: Instant,
    duration: Duration,
}

impl Elevator {
    pub fn new(floor: Floor) -> Elevator {
        Elevator {
            floor,
            state: State::Idle,
            requests: Requests::new(Floor::get_n_floors()),
            timer: None,
        }
    }
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
