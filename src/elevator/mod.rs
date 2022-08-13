pub mod event;
pub mod request;
pub mod state;
pub mod timer;

use self::request::Requests;
use self::state::State;
use self::timer::Timer;
use crate::error::ElevatorError;

pub struct Elevator {
    pub floor: usize,
    n_floors: usize,
    pub requests: Requests,
    pub state: State,
    pub timer: Option<Timer>,
}

impl Elevator {
    pub fn new(floor: usize, n_floors: usize) -> Elevator {
        let requests = Requests::new(n_floors);
        let state = State::Idle;
        let timer = None;
        Elevator {
            floor,
            n_floors,
            requests,
            state,
            timer,
        }
    }

    pub fn error(&self, critical: bool) -> ElevatorError {
        let (floor, state) = (self.floor, self.state);
        ElevatorError {
            floor,
            state,
            critical,
        }
    }

    pub fn get_n_floors(&self) -> usize {
        self.n_floors
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
