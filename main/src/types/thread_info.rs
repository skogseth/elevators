use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

use interface::types::{Direction, Floor};

use crate::error::ElevatorError;
use crate::state_machine::types::State;
use crate::types::{Message, ThreadInfo};

impl ThreadInfo {
    pub fn new(
        id: usize,
        handle: JoinHandle<Result<(), ElevatorError>>,
        transmitter: Sender<Message>,
    ) -> Self {
        ThreadInfo {
            id,
            handle,
            transmitter,
            floor: Floor::new(),
            state: State::Idle,
            n_requests: 0,
        }
    }

    pub fn cost_function(&self, floor: Floor, direction: Direction) -> usize {
        let in_direction = match self.state {
            State::Idle => true,
            State::Moving(dir) => direction == dir,
            State::Still(dir) => direction == dir,
        };
        let floor_difference = usize::from(floor).abs_diff(usize::from(self.floor));
        Self::cost_function_helper(self.state, floor_difference, self.n_requests, in_direction)
    }

    fn cost_function_helper(
        state: State,
        floor_difference: usize,
        n_requests: usize,
        in_direction: bool,
    ) -> usize {
        let state_value = match state {
            State::Idle => 0,
            State::Moving(..) => 1,
            State::Still(..) => 3,
        };
        state_value + (floor_difference) + 2 * (n_requests) + (!in_direction as usize)
    }
}
