use crate::types::Button;
use crate::types::Direction;

impl Button {
    pub fn iterator() -> impl Iterator<Item = Button> {
        [Button::HallUp, Button::HallDown, Button::Cab].iter().copied()
    }
}

impl From<Direction> for Button {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => Button::HallUp,
            Direction::Down => Button::HallDown,
        }
    }
}