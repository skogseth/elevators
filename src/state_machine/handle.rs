use crate::elevator::event::button::Button;
use crate::elevator::state::direction::Direction;
use crate::elevator::state::State;
use crate::elevator::timer::Timer;
use crate::elevator::Elevator;
use crate::network::send;
use crate::Message;

use std::cmp::Ordering;
use std::net::TcpStream;
use std::sync::mpsc::Sender;

const TIME_WAIT_ON_FLOOR: u64 = 3; // in seconds

pub fn arrive_at_floor(stream: &mut TcpStream, elevator: &mut Elevator, floor: usize) {
    elevator.floor = floor;

    if let Err(_e) = send::floor_indicator(stream, floor) {
        eprintln!("Failed to turn on light at floor {:?}", floor);
    }

    if let State::Moving(direction) = elevator.state {
        if let Ok(direction) = check_for_stop(elevator, direction) {
            if let Ok(_) = send::stop(stream) {
                elevator.state = State::Still(direction);
                elevator.timer = Some(Timer::from_secs(TIME_WAIT_ON_FLOOR));

                if let Err(_e) = send::door_open_light(stream, true) {
                    eprintln!("Failed to turn on door open light at floor {:?}", floor);
                }

                for button in [Button::Cab, Button::from(direction)] {
                    if let Err(_e) = send::order_button_light(stream, button, floor, false) {
                        eprintln!(
                            "Failed to turn off order light for button {:?} for floor {:?}",
                            button, floor
                        );
                    }
                }
            } else {
                eprintln!("Failed to stop at floor {:?}", floor);
            }
        } else {
            //eprintln!("Will not stop at floor {:?}", floor);
        }
    } else {
        eprintln!(
            "Arrived at floor {:?} without moving, state: {:?}",
            floor, elevator.state
        );
        // TODO? return Err(elevator.error(true));
    }
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
    match button {
        Button::Cab => elevator.requests.add_request(button, floor),
        _ => {
            // Can safely unwrap direction since Button::Cab has been handled
            let direction = Direction::try_from(button).unwrap();
            let msg = Message::Request { floor, direction };
            tx.send(msg).unwrap();
        }
    }
    if let Err(_e) = send::order_button_light(stream, button, floor, true) {
        eprintln!(
            "Failed to turn on order light for button {:?} for floor {:?}",
            button, floor
        );
    };
}

pub fn timer_timed_out(stream: &mut TcpStream, elevator: &mut Elevator) {
    elevator.timer = None;

    if let Err(_e) = send::door_open_light(stream, false) {
        eprintln!(
            "Failed to turn off door open light at floor {:?}",
            elevator.floor
        );
    }

    if let State::Still(direction) = elevator.state {
        if let Err(_e) = try_continue(stream, elevator, direction) {
            elevator.state = State::Idle;
        };
    } else if let State::Idle = elevator.state {
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
    if elevator
        .requests
        .check_in_direction(elevator.floor, direction)
    {
        if let Err(_e) = send::motor_direction(stream, direction) {
            eprintln!(
                "failed to move in direction {:?} from {:?}",
                direction, elevator.floor
            );
            return Err(true);
        } else {
            elevator.state = State::Moving(direction);
            return Ok(());
        }
    }
    Err(false)
}

pub fn try_move(stream: &mut TcpStream, elevator: &mut Elevator) -> Result<(), Critical> {
    if let Some((floor, button)) = elevator.requests.check_for_any() {
        let direction = match elevator.floor.cmp(&floor) {
            Ordering::Less => Direction::Up,
            Ordering::Greater => Direction::Down,
            Ordering::Equal => {
                let directions = match Direction::try_from(button).ok() {
                    Some(direction) => vec![direction],
                    None => vec![Direction::Up, Direction::Down],
                };
                for direction in directions {
                    elevator.requests.request_at_floor(floor, direction);
                }

                elevator.timer = Some(Timer::from_secs(TIME_WAIT_ON_FLOOR));
                if let Err(_e) = send::door_open_light(stream, true) {
                    eprintln!("Failed to turn on door open light at floor {:?}", floor);
                }
                if let Err(_e) = send::order_button_light(stream, button, floor, false) {
                    eprintln!(
                        "Failed to turn off order light for button {:?} for floor {:?}",
                        button, floor
                    );
                }
                return Ok(());
            }
        };
        if let Err(_e) = send::motor_direction(stream, direction) {
            eprintln!(
                "failed to move in direction {:?} from {:?}",
                direction, elevator.floor
            );
            return Err(true);
        } else {
            elevator.state = State::Moving(direction);
            return Ok(());
        }
    }
    Err(false)
}
