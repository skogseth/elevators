use crate::types::{Button, Direction};

impl Button {
    pub fn iterator() -> impl Iterator<Item = Button> {
        [
            Button::Cab,
            Button::Hall(Direction::Up),
            Button::Hall(Direction::Down),
        ]
        .iter()
        .copied()
    }
}

impl From<Button> for u8 {
    fn from(button: Button) -> u8 {
        match button {
            Button::Hall(Direction::Up) => 0,
            Button::Hall(Direction::Down) => 1,
            Button::Cab => 2,
        }
    }
}

impl TryFrom<u8> for Button {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        let button = match val {
            0 => Button::Hall(Direction::Up),
            1 => Button::Hall(Direction::Down),
            2 => Button::Cab,
            _ => return Err("Failed to map u8 to button"),
        };
        Ok(button)
    }
}