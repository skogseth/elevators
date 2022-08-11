pub mod direction;

use self::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Idle,
    Moving(Direction),
    Still(Direction),
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            State::Idle => format!("State: Idle"),
            State::Moving(dir) => format!("State: Moving ({dir})"),
            State::Still(dir) => format!("State: Moving ({dir})"),
        };
        write!(f, "{s}")
    }
}