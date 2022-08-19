pub mod button;
pub mod direction;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Button {
    HallUp = 0,
    HallDown = 1,
    Cab = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up = 1,
    Down = -1,
}
