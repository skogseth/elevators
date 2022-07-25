use crate::state_machine::request::Request;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

pub struct Elevator {
    pub floor: usize,
    pub requests: Vec<Request>,
}

impl Elevator {
    pub fn new(floor: usize) -> Elevator {
        let requests: Vec<Request> = Vec::new();
        Elevator {floor, requests}
    }
}