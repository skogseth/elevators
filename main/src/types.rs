use std::thread::JoinHandle;
use std::sync::mpsc::Sender;

use interface::types::{Direction, Floor};

use crate::state_machine::types::State;
use crate::error::ElevatorError;

pub mod elevator;
pub mod thread_info;

use self::elevator::{Requests, Timer};

pub struct Elevator {
    pub floor: Floor,
    pub state: State,
    pub requests: Requests,
    pub timer: Option<Timer>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    Request {
        floor: Floor,
        direction: Direction,
    },
    HallButtonLight {
        floor: Floor,
        direction: Direction,
        on: bool,
    },
    ElevatorInfo {
        thread_id: usize,
        floor: Floor,
        state: State,
        n_requests: usize,
    },
    Shutdown,
}

pub struct ThreadInfo {
    pub id: usize,
    pub handle: JoinHandle<Result<(), ElevatorError>>,
    pub transmitter: Sender<Message>,
    pub floor: Floor,
    pub state: State,
    pub n_requests: usize,
}