use crate::elevator::state::State;

#[derive(Debug, Clone, Copy)]
pub struct ElevatorError {
    pub floor: usize,
    pub state: State,
    pub critical: bool,
}

impl std::fmt::Display for ElevatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error occurred at floor {} in state {}", self.floor, self.state)
    }
}

impl std::error::Error for ElevatorError {}

pub trait Logger {
    fn log_if_err(self);
}

impl Logger for Result<(), std::io::Error> {
    fn log_if_err(self) {
        if let Err(e) = self {
            eprintln!("{:?}", e);
        }
    } 
}