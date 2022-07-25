use crate::request::Request;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
}

pub struct Elevator {
    floor: usize,
    requests: Vec<Request>,
}

impl Elevator {
    pub fn new(floor: usize) -> Elevator {
        let requests: Vec<Request> = Vec::new();
        Elevator {floor, requests}
    }

    pub fn get_floor(&self) -> usize { self.floor }
    pub fn set_floor(&mut self, floor: usize) { self.floor = floor }

    pub fn pull_request(&mut self) -> Option<Request> { self.requests.pop() }
    pub fn push_request(&mut self, request Request) { self.requests.pull() }
}