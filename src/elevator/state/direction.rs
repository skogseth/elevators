#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up = 1,
    Down = -1,
}

impl Direction {
    fn to_str(&self) -> &str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}