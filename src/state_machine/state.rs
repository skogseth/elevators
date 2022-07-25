use crate::elevator::{Direction, Elevator};

pub enum State {
    Idle(Elevator),
    Moving(Elevator, Direction),
    Still(Elevator, Option<Direction>),
}