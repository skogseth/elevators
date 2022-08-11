pub mod button;

use self::button::Button;

pub enum Event {
    ArriveAtFloor(usize),
    ButtonPress(Button, usize),
    TimerTimedOut,
}