use crate::elevator::state::{direction::Direction, State};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Request {
        floor: usize,
        direction: Direction,
    },
    HallButtonLight {
        floor: usize,
        direction: Direction,
        on: bool,
    },
    ElevatorInfo {
        thread_id: usize,
        floor: usize,
        state: State,
        n_requests: usize,
    },
    Shutdown,
}