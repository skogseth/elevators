use crate::elevator::event::button::Button;
use crate::elevator::state::direction::Direction;
use crate::elevator::state::State;
use crate::elevator::timer::Timer;
use crate::elevator::Elevator;
use crate::network::send;

use std::net::TcpStream;

const TIME_WAIT_ON_FLOOR: u64 = 3; // in seconds

pub fn arrive_at_floor(stream: &mut TcpStream, elevator: &mut Elevator, floor: usize) {
    if let Err(_e) = send::floor_indicator(stream, floor) {
        eprintln!("Failed to turn on light at floor {:?}", floor);
    }
    if let State::Moving(direction) = elevator.state {
        elevator.floor = floor;

        if elevator.requests.request_at_floor(floor, direction) {
            if let Err(_e) = send::stop(stream) {
                eprintln!("Failed to stop at floor {:?}", floor);
            } else {
                elevator.state = State::Still(direction);
                elevator.timer = Some(Timer::from_secs(TIME_WAIT_ON_FLOOR));
                if let Err(_e) = send::door_open_light(stream, true) {
                    eprintln!("Failed to turn on door open light at floor {:?}", floor);
                }
                for button in Button::iterator() {
                    if let Err(_e) = send::order_button_light(stream, button, floor, false) {
                        eprintln!("Failed to turn off order light for button {:?} for floor {:?}", button, floor);
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

pub fn button_press(stream: &mut TcpStream, elevator: &mut Elevator, button: Button, floor: usize) {
    elevator.requests.add_request(button, floor);
    if let Err(_e) = send::order_button_light(stream, button, floor, true) {
        eprintln!("Failed to turn on order light for button {:?} for floor {:?}", button, floor);
    }
    if let State::Idle = elevator.state {
        for direction in [Direction::Up, Direction::Down] {
            if elevator
                .requests
                .check_for_requests(elevator.floor, direction)
            {
                println!("Trying to move in direction {direction:?}");
                if let Err(_e) = send::motor_direction(stream, direction) {
                    let floor_msg = format!("from floor {:?} to floor {:?}", elevator.floor, floor);
                    eprintln!("failed to move in direction {:?} {}", direction, floor_msg);
                } else {
                    elevator.state = State::Moving(direction);
                }
            }
        }
    }
}

pub fn timer_timed_out(stream: &mut TcpStream, elevator: &mut Elevator) {
    elevator.timer = None;
    if let Err(_e) = send::door_open_light(stream, false) {
        eprintln!("Failed to turn off door open light at floor {:?}", elevator.floor);
    }
    if let State::Still(direction) = elevator.state {
        let directions = [
            direction,
            match direction {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
            },
        ];

        for direction in directions {
            if elevator
                .requests
                .check_for_requests(elevator.floor, direction)
            {
                if let Err(_e) = send::motor_direction(stream, direction) {
                    eprintln!(
                        "failed to move in direction {:?} from {:?}",
                        direction, elevator.floor
                    );
                } else {
                    elevator.state = State::Moving(direction);
                    return;
                }
            }
        }

        elevator.state = State::Idle;
    }
}
