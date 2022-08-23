use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;

use interface::send;
use interface::types::{Button, Direction, Floor};

use crate::error::{ElevatorError, Logger};
use crate::types::elevator::Timer;
use crate::types::{Elevator, Message};

use super::types::State;

type Critical = bool;

const TIME_WAIT_ON_FLOOR: u64 = 3; // in seconds

pub async fn arrive_at_floor(
    stream: &mut TcpStream,
    tx: &Sender<Message>,
    elevator: &mut Elevator,
    floor: Floor,
) {
    elevator.floor = floor;

    send::floor_indicator(stream, elevator.floor)
        .await
        .log_if_err();

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

    if let Err(_) = send::stop(stream).await {
        eprintln!("Failed to stop at floor {:?}", elevator.floor);
        return;
    }

    wait_at_floor(stream, tx, elevator, direction).await;
}

pub async fn message_received(
    stream: &mut TcpStream,
    elevator: &mut Elevator,
    msg: Message,
) -> Result<(), ElevatorError> {
    match msg {
        Message::Request { floor, direction } => {
            let button = Button::Hall(direction);
            elevator.requests.add_request(button, floor);
        }
        Message::HallButtonLight {
            floor,
            direction,
            on,
        } => {
            let button = Button::Hall(direction);
            send::order_button_light(stream, button, floor, on)
                .await
                .log_if_err();
            elevator.requests.update_active_button(button, floor, !on);
        }
        Message::ElevatorInfo { .. } => {
            eprintln!("Main thread sent elevator info...");
        }
        Message::Shutdown => return Err(elevator.error(false)),
    }

    Ok(())
}

pub async fn button_press(
    stream: &mut TcpStream,
    tx: &Sender<Message>,
    elevator: &mut Elevator,
    button: Button,
    floor: Floor,
) {
    match button {
        Button::Cab => {
            elevator.requests.add_request(button, floor);
            send::order_button_light(stream, button, floor, true)
                .await
                .log_if_err();
            elevator.requests.update_active_button(button, floor, false);
        }
        Button::Hall(direction) => {
            // Send hall button light message to main thread
            let msg = Message::HallButtonLight {
                floor,
                direction,
                on: true,
            };
            tx.send(msg).await.unwrap();

            // Send request to main thread
            let msg = Message::Request { floor, direction };
            tx.send(msg).await.unwrap();
        }
    }
}

pub async fn timer_timed_out(
    stream: &mut TcpStream,
    tx: &Sender<Message>,
    elevator: &mut Elevator,
) {
    elevator.timer = None;

    send::door_open_light(stream, false).await.log_if_err();

    if let State::Still(direction) = elevator.state {
        if let Ok(direction) = check_for_stop(elevator, direction) {
            wait_at_floor(stream, tx, elevator, direction).await;
            return;
        }

        if let Err(_) = try_continue(stream, elevator, direction).await {
            elevator.state = State::Idle;
        };
    } else {
        eprintln!("Timer timed out, but state was not still");
    }
}

pub async fn try_move(
    stream: &mut TcpStream,
    tx: &Sender<Message>,
    elevator: &mut Elevator,
) -> Result<(), Critical> {
    println!("Trying to move");

    for direction in Direction::iterator() {
        if let Ok(direction) = check_for_stop(elevator, direction) {
            println!("Found request at current floor, direction: {direction}");
            wait_at_floor(stream, tx, elevator, direction).await;
            return Ok(());
        }
    }

    let direction = check_in_both_directions(&elevator).map_err(|_| false)?;

    println!("Request found in direction: {direction}");

    if let Err(e) = send::motor_direction(stream, direction).await {
        eprintln!("{e}");
        return Err(true);
    };

    println!("Succesfully sent motor direction: {direction}");

    elevator.state = State::Moving(direction);
    Ok(())
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
    let mut requests_at_floor_in_direction =
        requests.get_requests_at_floor(elevator.floor, direction);
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
            .get_requests_at_floor(elevator.floor, direction.opposite())
            .is_empty()
        {
            return Ok(direction.opposite());
        }
    }

    // Returns Ok(direction) if there was a cab request
    // Returns Err(()) if no request was found
    result
}

fn check_in_both_directions(elevator: &Elevator) -> Result<Direction, ()> {
    for direction in Direction::iterator() {
        if elevator
            .requests
            .check_in_direction(elevator.floor, direction)
        {
            return Ok(direction);
        }
    }
    Err(())
}

async fn wait_at_floor(
    stream: &mut TcpStream,
    tx: &Sender<Message>,
    elevator: &mut Elevator,
    direction: Direction,
) {
    elevator.state = State::Still(direction);
    elevator.timer = Some(Timer::from_secs(TIME_WAIT_ON_FLOOR));

    // wait for a short duration to give the button lights some time to shine, literally
    sleep(Duration::from_millis(50)).await;

    send::door_open_light(stream, true).await.log_if_err();
    send::order_button_light(stream, Button::Cab, elevator.floor, false)
        .await
        .log_if_err();
    elevator
        .requests
        .update_active_button(Button::Cab, elevator.floor, true);

    let msg = Message::HallButtonLight {
        floor: elevator.floor,
        direction,
        on: false,
    };
    tx.send(msg).await.unwrap();
}

async fn try_continue(
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

    if let Err(_) = send::motor_direction(stream, direction).await {
        eprintln!(
            "failed to move in direction {:?} from {:?}",
            direction, elevator.floor
        );
        return Err(true);
    }

    elevator.state = State::Moving(direction);
    return Ok(());
}
