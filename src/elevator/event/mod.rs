pub mod button;

use self::button::Button;
use crate::Message;

pub enum Event {
    ArriveAtFloor(usize),
    TimerTimedOut,
    MessageReceived(Message),
    ButtonPress(Button, usize),
}