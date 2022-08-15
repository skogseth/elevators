#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Button {
    HallUp = 0,
    HallDown = 1,
    Cab = 2,
}

use self::Button::*;
use crate::elevator::state::direction::Direction;

impl Button {
    pub fn iterator() -> impl Iterator<Item = Button> {
        [HallUp, HallDown, Cab].iter().copied()
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