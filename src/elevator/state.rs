pub enum State {
    Idle,
    Moving,
    Still,
}

impl State {
    fn to_str(&self) -> &str {
        match self {
            State::Idle => "idle",
            State::Moving => "moving",
            State::Still => "still",
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}