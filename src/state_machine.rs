use std::io;

mod event;
mod state;

use crate::state_machine::{event::Event, state::State};
use crate::elevator::Elevator;

pub fn go(_thread: usize) -> Result<(), io::Error> {
    let event = Event::TimerTimedOut;
    let _state = State::Idle(Elevator::new(0));

    loop { //replace loop with wait for event
        match event {
            Event::ButtonPress(_floor) => {
                //add request for floor, and optionally change state
            }
            Event::ArriveAtFloor(_floor) => {
                //add request for floor, and optionally change state
            }
            Event::TimerTimedOut => {
                //shrug
            }
        }

        break;
    }

    Ok(())
}