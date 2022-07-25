use std::io;

mod elevator;
mod request;

use crate::state_machine::elevator::{Elevator, Direction};

enum State {
    Idle(Elevator),
    Moving(Elevator, Direction),
    Still(Elevator, Option<Direction>),
}

pub fn go(_thread: usize) -> Result<(), io::Error> {
    let state = State::Idle(Elevator::new(0));

    loop {
        match state {
            State::Idle(_elevator) => println!("Elevator idle"),//check requests
            State::Moving(_elevator, dir) => println!("Elevator moving in direction {dir:?}"),
            State::Still(_elevator, opt_dir) => {
                match opt_dir {
                    Some(dir) => println!("Elevator standing still, going in direction {dir:?}"),
                    None => println!("Elevator standing still"),
                }
            }
        }

        break;
    }

    Ok(())
}