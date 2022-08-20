pub mod button;
pub mod direction;
pub mod floor;

/// Type representation for button values
/// Button::Hall(Direction::Up) <==> 0
/// Button::Hall(Direction::Down) <==> 1
/// Button::Cab <==> 2
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {
    Hall(Direction),
    Cab,
}

/// Type representation for direction values
/// Direction::Up <==> 1
/// Direction::Down <==> -1 (255)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
}

/// Type representation for floor values
/// The max value is kept as a static variable that is initialized once,
/// via the Floor::initialize(max_floor) function, and the immutable.
/// Is read under constructor calls to the Floor-type, eg. Floor::new().
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Floor {
    val: usize,
    max: usize,
}
