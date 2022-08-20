pub mod button;
pub mod direction;
pub mod floor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    Hall(Direction),
    Cab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
}

// Currently not implemented
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Floor {
    val: usize,
    max: usize,
}
