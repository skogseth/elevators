use crate::types::Direction;

impl Direction {
    fn to_str(&self) -> &str {
        match self {
            Direction::Up => "up",
            Direction::Down => "down",
        }
    }

    pub fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    pub fn iterator() -> impl Iterator<Item = Direction> {
        [Direction::Up, Direction::Down].iter().copied()
    }
}

impl From<Direction> for u8 {
    fn from(val: Direction) -> u8 {
        match val {
            Direction::Up => 1,
            Direction::Down => 255, // = -1
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
