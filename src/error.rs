#[derive(Debug)]
pub struct ElevatorError;

impl std::fmt::Display for ElevatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "error occurred for elevator")
    }
}

impl std::error::Error for ElevatorError {}
