use crate::elevator::button::Button;

pub enum Event {
    ArriveAtFloor(usize),
    ButtonPress(Button, usize),
    TimerTimedOut,
}