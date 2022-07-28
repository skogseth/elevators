#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
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