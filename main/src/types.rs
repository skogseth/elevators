use std::thread::JoinHandle;
use std::sync::mpsc::Sender;

use interface::types::Direction;

use crate::state_machine::types::State;
use crate::error::ElevatorError;

pub mod elevator;
pub mod thread_info;

use self::elevator::{Requests, Timer};

pub struct Elevator {
    pub floor: usize,
    n_floors: usize,
    pub requests: Requests,
    pub state: State,
    pub timer: Option<Timer>,
}

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

pub struct ThreadInfo {
    pub id: usize,
    pub handle: JoinHandle<Result<(), ElevatorError>>,
    pub transmitter: Sender<Message>,
    pub floor: usize,
    pub state: State,
    pub n_requests: usize,
}