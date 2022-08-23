use tokio::sync::mpsc::Sender;

use interface::types::{Direction, Floor};

use crate::state_machine::types::State;

pub mod elevator;
pub mod task_info;

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
        task_id: usize,
        floor: Floor,
        state: State,
        n_requests: usize,
    },
    Shutdown,
}

pub struct TaskInfo {
    pub id: usize,
    pub transmitter: Sender<Message>,
    pub floor: Floor,
    pub state: State,
    pub n_requests: usize,
}