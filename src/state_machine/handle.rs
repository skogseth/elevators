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
    if let Err(_e) = send::floor_indicator(stream, floor) {
        eprintln!("Failed to turn on light at floor {:?}", floor);
    }
    if let State::Moving(direction) = elevator.state {
        elevator.floor = floor;

        let stop = {
            let requests = &mut elevator.requests;
            let stop_for_direction = requests.request_at_floor(floor, direction);
            let stop_for_opposite_direction = !requests.check_in_direction(floor, direction)
                && requests.request_at_floor(floor, direction.opposite());

            if stop_for_opposite_direction {
                Ok(direction.opposite())
            } else if stop_for_direction {
                Ok(direction)
            } else {
                Err(())
            }
        };

        if let Ok(direction) = stop {
            if let Err(_e) = send::stop(stream) {
                eprintln!("Failed to stop at floor {:?}", floor);
            } else {
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
            }
        }
    } else {
        eprintln!(
            "Arrived at floor {:?} without moving, state: {:?}",
            floor, elevator.state
        );
        // TODO? return Err(elevator.error(true));
    }
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
