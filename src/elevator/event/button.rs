#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Button {
    HallUp = 0,
    HallDown = 1,
    Cab = 2,
}

use self::Button::*;

impl Button {
    pub fn iterator() -> impl Iterator<Item = Button> {
        [HallUp, HallDown, Cab].iter().copied()
    }
}