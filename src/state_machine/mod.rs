use std::net::TcpStream;
use std::sync::mpsc::Receiver;

mod handle;

use crate::elevator::event::{button::Button, Event};
use crate::elevator::Elevator;
use crate::error::ElevatorError;
use crate::network::get;
use crate::Message;

const TIMEOUT: u128 = 10;

pub fn run(
    mut stream: TcpStream,
    _rx: Receiver<Message>,
    n_floors: usize,
) -> Result<(), ElevatorError> {
    let start_floor = get::floor(&mut stream).unwrap().unwrap();
    let mut elevator = Elevator::new(start_floor, n_floors);

    loop {
        let events = wait_for_event(&mut stream, &elevator);

        for event in events {
            match event {
                Event::ArriveAtFloor(floor) => {
                    handle::arrive_at_floor(&mut stream, &mut elevator, floor);
                }
                Event::ButtonPress(button, floor) => {
                    handle::button_press(&mut stream, &mut elevator, button, floor);
                }
                Event::TimerTimedOut => {
                    handle::timer_timed_out(&mut stream, &mut elevator);
                }
            }
        }
    }
}

fn wait_for_event(stream: &mut TcpStream, elevator: &Elevator) -> Vec<Event> {
    //let now = Instant::now();
    println!("Waiting for event... (elevator state: {:?}))", elevator.state);
    let mut events = Vec::new();
    loop {
        // CHECK FOR FLOOR ARRIVE EVENT
        if let Ok(opt_floor) = get::floor(stream) {
            if let Some(floor) = opt_floor {
                if floor != elevator.floor {
                    println!(
                        "Arrival at floor {}, current_floor {}",
                        floor, elevator.floor
                    );
                    events.push(Event::ArriveAtFloor(floor));
                }
            }
        } else {
            eprintln!("caught error in get::floor()!");
            // TODO
        }

        // CHECK FOR BUTTON PRESS
        let button = Button::Cab;
        let floors = elevator
            .requests
            .get(&button)
            .iter()
            .enumerate()
            .filter(|&x| *x.1 == false);
        for (floor, _) in floors {
            if let Ok(pressed) = get::order_button(stream, button, floor) {
                if pressed {
                    events.push(Event::ButtonPress(button, floor));
                    println!("Button {:?} was pressed at floor {:?}", button, floor);
                }
            } else {
                let identifier = format!("floor {floor:?} & button {button:?}");
                eprintln!("caught error in get::order_button() for {identifier}");
                // TODO
            }
        }

        // CHECK FOR TIMER
        if let Some(timer) = elevator.timer {
            if timer.is_done() {
                events.push(Event::TimerTimedOut);
            }
        }

        // CHECK FOR MESSAGES
        /*
        // TODO
         */

        if events.len() > 0 {
            break;
        }
    }
    events
}