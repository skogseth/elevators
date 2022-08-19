use crate::types::Button;
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
}

impl TryFrom<Button> for Direction {
    type Error = &'static str;

    fn try_from(button: Button) -> Result<Self, Self::Error> {
        match button {
            Button::HallUp => Ok(Direction::Up),
            Button::HallDown => Ok(Direction::Down),
            Button::Cab => Err("Button::Cab has no direction"),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}