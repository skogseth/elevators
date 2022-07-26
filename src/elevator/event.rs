pub enum Event {
    ButtonPress(usize),
    ArriveAtFloor(usize),
    TimerTimedOut,
}