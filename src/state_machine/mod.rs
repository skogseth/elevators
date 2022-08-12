use std::net::TcpStream;
use std::time::Instant;

mod handle;

use crate::elevator::Elevator;
use crate::elevator::event::{Event, button::Button};
use crate::elevator::state::{State, direction::Direction};
use crate::error::ElevatorError;
use crate::network::{get, send};

const TIMEOUT: u128 = 10;

pub fn run(mut stream: TcpStream, n_floors: usize) -> Result<(), ElevatorError> {
    let start_floor = get::floor(&mut stream).unwrap().unwrap();
    let mut elevator = Elevator::new(start_floor, n_floors);
    elevator.state = State::Moving(Direction::Up);

    loop {
        let events = wait_for_event(&mut stream, elevator.floor, n_floors);
        
        for event in events {
            match event {
                Event::ArriveAtFloor(floor) => {
                    handle::arrive_at_floor(&mut stream, &mut elevator, floor);
                }
                Event::ButtonPress(button, floor) => {
                    handle::button_press(&mut elevator, button, floor);
                }
                Event::TimerTimedOut => {
                    handle::timer_timed_out(&mut elevator);
                }
            }
        }
    }

    Ok(())
}



fn wait_for_event(stream: &mut TcpStream, current_floor: usize, n_floors: usize) -> Vec<Event> {
    //let now = Instant::now();
    let mut events = Vec::new();
    loop {
        // CHECK FOR FLOOR ARRIVE EVENT
        match get::floor(stream) {
            Ok(opt_floor) => if let Some(floor) = opt_floor {
                if floor != current_floor {
                    events.push(Event::ArriveAtFloor(floor));
                }
            }
            Err(_e) => {
                eprintln!("caught error in get::floor()!");
                // TODO
            }
        }
    
        // CHECK FOR BUTTON PRESS
        for floor in 0..n_floors {
            for button in Button::iterator() {
                if let Ok(_pressed) = get::order_button(stream, button, floor) {
                    events.push(Event::ButtonPress(button, floor));
                } else {
                    eprintln!("caught error in get::order_button() for floor {floor:?} & button {button:?}");
                    // TODO
                }
            }
        }

        // CHECK FOR TIMER
        /*
        if now.elapsed().as_micros() > TIMEOUT {
            return vec![Event::TimerTimedOut];
        }
        */

        if events.len() > 0 {
            break;
        }
    }
    events
}