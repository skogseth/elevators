use crate::elevator::event::button::Button;
use crate::elevator::state::direction::Direction;
use crate::elevator::state::State;
use crate::elevator::timer::Timer;
use crate::elevator::Elevator;
use crate::error::Logger;
use crate::network::send;
use crate::Message;

use std::cmp::Ordering;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

const TIME_WAIT_ON_FLOOR: u64 = 3; // in seconds

pub fn arrive_at_floor(stream: &mut TcpStream, tx: &Sender<Message>, elevator: &mut Elevator) {
    send::floor_indicator(stream, elevator.floor).log_if_err();

    let direction = match elevator.state {
        State::Moving(dir) => dir,
        _ => {
            let (floor, state) = (elevator.floor, elevator.state);
            eprintln!("Arrived at floor {floor} without moving (State::{state:?})");
            return;
        }
    };

    let direction = match check_for_stop(elevator, direction) {
        Ok(dir) => dir,
        Err(_) => return,
    };

    if let Err(_) = send::stop(stream) {
        eprintln!("Failed to stop at floor {:?}", elevator.floor);
        return;
    }

    elevator.state = State::Still(direction);
    elevator.timer = Some(Timer::from_secs(TIME_WAIT_ON_FLOOR));

    send::door_open_light(stream, true).log_if_err();
    send::order_button_light(stream, Button::Cab, elevator.floor, false).log_if_err();

    let msg = Message::HallButtonLight {
        floor: elevator.floor,
        direction,
        on: false,
    };
    tx.send(msg).unwrap();
}

// Checks for stop at current floor of elevator, with a prioritized direction
//
// If there is any reason to stop, the function returns Ok(Direction),
// where direction is the prioritized direction after stop.
// If not the function returns an empty error: Err(())
//
// The priority of requests is as follows:
// Hall request in the correct direction => direction
// Hall request in the opposite direction and no more requests in the current direction => opposite direction
// Cab request => direction
fn check_for_stop(elevator: &mut Elevator, direction: Direction) -> Result<Direction, ()> {
    let requests = &mut elevator.requests;
    let mut requests_at_floor_in_direction = requests.request_at_floor(elevator.floor, direction);
    let mut result = Err(());

    // Check if any requests at floor in direction
    // If the request is a hall request return with direction as result
    // If the request is a cab request we set direction as temporary result
    if !requests_at_floor_in_direction.is_empty() {
        while let Some(button) = requests_at_floor_in_direction.pop() {
            if button != Button::Cab {
                return Ok(direction);
            }
        }
        result = Ok(direction);
    }

    // Check if there are no more requests in direction
    // If true, check if there are any requests at the floor in the opposite direction
    // If true, return with opposite direction as result
    if !requests.check_in_direction(elevator.floor, direction) {
        if !requests
            .request_at_floor(elevator.floor, direction.opposite())
            .is_empty()
        {
            return Ok(direction.opposite());
        }
    }

    // Returns Ok(direction) if there was a cab request
    // Returns Err(()) if no request was found
    result
}

pub fn button_press(
    stream: &mut TcpStream,
    tx: &Sender<Message>,
    elevator: &mut Elevator,
    button: Button,
    floor: usize,
) {
    if button == Button::Cab {
        elevator.requests.add_request(button, floor);
        send::order_button_light(stream, button, floor, true).log_if_err();
    } else {
        // Can safely unwrap direction since Button::Cab has been handled
        let direction = Direction::try_from(button).unwrap();

        // Send request to main thread
        let msg = Message::Request { floor, direction };
        tx.send(msg).unwrap();

        // Send hall button light message to main thread
        let msg = Message::HallButtonLight { floor, direction, on: true };
        tx.send(msg).unwrap();
    }
}

pub fn timer_timed_out(stream: &mut TcpStream, elevator: &mut Elevator) {
    elevator.timer = None;

    send::door_open_light(stream, false).log_if_err();

    if let State::Still(direction) = elevator.state {
        if let Err(_) = try_continue(stream, elevator, direction) {
            elevator.state = State::Idle;
        };
    } else {
        eprintln!("Timer timed out, but state was not still");
    }
}

type Critical = bool;

fn try_continue(
    stream: &mut TcpStream,
    elevator: &mut Elevator,
    direction: Direction,
) -> Result<(), Critical> {
    let continue_in_direction = elevator
        .requests
        .check_in_direction(elevator.floor, direction);

    if !continue_in_direction {
        return Err(false);
    }

    match send::motor_direction(stream, direction) {
        Ok(_) => {
            elevator.state = State::Moving(direction);
            return Ok(());
        }
        Err(_) => {
            eprintln!(
                "failed to move in direction {:?} from {:?}",
                direction, elevator.floor
            );
            return Err(true);
        }
    }
}

pub fn try_move(stream: &mut TcpStream, elevator: &mut Elevator) -> Result<(), Critical> {
    let (floor, button) = match elevator.requests.check_for_any() {
        Some(tuple) => tuple,
        None => return Err(false),
    };

    let direction = match elevator.floor.cmp(&floor) {
        Ordering::Less => Direction::Up,
        Ordering::Greater => Direction::Down,
        Ordering::Equal => {
            let directions = match Direction::try_from(button).ok() {
                Some(direction) => vec![direction],
                None => vec![Direction::Up, Direction::Down],
            };

            for direction in directions {
                if !elevator
                    .requests
                    .request_at_floor(floor, direction)
                    .is_empty()
                {
                    break;
                }
            }

            elevator.timer = Some(Timer::from_secs(TIME_WAIT_ON_FLOOR));
            send::door_open_light(stream, true).log_if_err();
            send::order_button_light(stream, button, floor, false).log_if_err();
            return Ok(());
        }
    };

    match send::motor_direction(stream, direction) {
        Ok(_) => {
            elevator.state = State::Moving(direction);
            Ok(())
        }
        Err(e) => {
            eprintln!("{e}");
            Err(true)
        }
    }
}
