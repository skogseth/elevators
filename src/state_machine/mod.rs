use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};

mod handle;

use crate::elevator::event::{button::Button, Event};
use crate::elevator::Elevator;
use crate::elevator::state::State;
use crate::error::ElevatorError;
use crate::network::get;
use crate::Message;

const TIMEOUT: u128 = 10;

pub fn run(
    mut stream: TcpStream,
    (tx, rx): (Sender<Message>, Receiver<Message>),
    n_floors: usize,
) -> Result<(), ElevatorError> {
    let start_floor = get::floor(&mut stream).unwrap().unwrap();
    let mut elevator = Elevator::new(start_floor, n_floors);

    loop {
        let event = wait_for_event(&mut stream, &rx, &elevator);

        match event {
            Event::ArriveAtFloor(floor) => {
                handle::arrive_at_floor(&mut stream, &mut elevator, floor);
            }
            Event::TimerTimedOut => {
                handle::timer_timed_out(&mut stream, &mut elevator);
            }
            Event::MessageReceived(msg) => match msg {
                Message::Request { floor, direction } => {
                    let button = Button::from(direction);
                    elevator.requests.add_request(button, floor);
                }
                Message::Shutdown => return Ok(()),
            },
            Event::ButtonPress(button, floor) => {
                handle::button_press(&mut stream, &tx, &mut elevator, button, floor);
            }
        }

        if elevator.state == State::Idle {
            handle::try_move(&mut stream, &mut elevator).err();
        }
    }
}

fn wait_for_event(stream: &mut TcpStream, rx: &Receiver<Message>, elevator: &Elevator) -> Event {
    println!(
        "Waiting for event... (State::{:?}))",
        elevator.state
    );
    loop {
        // CHECK FOR FLOOR ARRIVAL
        if let Ok(opt_floor) = get::floor(stream) {
            if let Some(floor) = opt_floor {
                if floor != elevator.floor {
                    println!("Arrival at floor {floor}");
                    return Event::ArriveAtFloor(floor);
                }
            }
        } else {
            eprintln!("caught error in get::floor()!");
        }

        // CHECK FOR TIMER
        if let Some(timer) = elevator.timer {
            if timer.is_done() {
                eprintln!("Timer finished");
                return Event::TimerTimedOut;
            }
        }

        // CHECK FOR MESSAGES
        if let Ok(msg) = rx.try_recv() {
            eprintln!("Message received: {:?}", msg);
            return Event::MessageReceived(msg);
        }

        // CHECK FOR BUTTON PRESS
        for button in Button::iterator() {
            let requests = elevator.requests.get(&button);
            let floors = requests
                .iter()
                .enumerate()
                .filter(|&x| *x.1 == false)
                .map(|x| x.0);
            for floor in floors {
                if let Ok(pressed) = get::order_button(stream, button, floor) {
                    if pressed {
                        println!("Button {:?} was pressed at floor {:?}", button, floor);
                        return Event::ButtonPress(button, floor);
                    }
                } else {
                    let identifier = format!("floor {floor:?} & button {button:?}");
                    eprintln!("caught error in get::order_button() for {identifier}");
                }
            }
        }
    }
}
