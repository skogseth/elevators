use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;

use interface::types::{Button, Direction, Floor};
use interface::{get, send};

use crate::error::Error;
use crate::types::{Elevator, Message};

mod handle;
pub mod types;

use self::types::{Event, State};

const TIME_BETWEEN_EVENT_CHECKS: u64 = 10; // in milliseconds

pub async fn run(
    task_id: usize,
    mut stream: TcpStream,
    (tx, mut rx): (Sender<Message>, Receiver<Message>),
) -> Result<(), Error> {
    let start_floor = initialize(&mut stream).await.map_err(|e| Error::StartUp(e))?;
    let mut elevator = Elevator::new(start_floor);

    loop {
        // Send elevatorinfo to main task
        let msg = Message::ElevatorInfo {
            task_id,
            floor: elevator.floor,
            state: elevator.state,
            n_requests: elevator.requests.number_of_requests(),
        };
        tx.send(msg).await.unwrap();

        // Wait for event
        let event = tokio::select! {
            result = elevator_event(task_id, &mut stream, &elevator) => { 
                match result {
                    Ok(event) => event,
                    Err(e) => {
                        println!("Error in elevator_event(): {e}");
                        continue;
                    }
                }
            }
            option = rx.recv() => {
                let msg = option.ok_or(Error::ChannelShutdown)?;
                eprintln!("Task {task_id}: Message received: {:?}", msg);
                Event::MessageReceived(msg)
            }
        };

        match event {
            Event::ArriveAtFloor(floor) => {
                handle::arrive_at_floor(&mut stream, &tx, &mut elevator, floor).await;
            }
            Event::TimerTimedOut => {
                handle::timer_timed_out(&mut stream, &tx, &mut elevator).await;
            }
            Event::MessageReceived(msg) => {
                handle::message_received(&mut stream, &mut elevator, msg).await?;
            }
            Event::ButtonPress(button, floor) => {
                handle::button_press(&mut stream, &tx, &mut elevator, button, floor).await;
            }
        }

        if elevator.state == State::Idle {
            handle::try_move(&mut stream, &tx, &mut elevator)
                .await
                .err();
        }
    }
}

async fn elevator_event(
    task_id: usize,
    stream: &mut TcpStream,
    elevator: &Elevator,
) -> Result<Event, std::io::Error> {
    println!(
        "Task {task_id}: Waiting for event... (State::{:?}))",
        elevator.state
    );

    loop {
        // CHECK FOR FLOOR ARRIVAL
        if let State::Moving(_) = elevator.state {
            if let Some(floor) = get::floor(stream).await? {
                if floor != elevator.floor {
                    println!("task {task_id}: Arrival at floor {floor}");
                    return Ok(Event::ArriveAtFloor(floor));
                }
            }
        }

        // CHECK FOR TIMER
        if let Some(timer) = elevator.timer {
            if timer.is_done() {
                eprintln!("task {task_id}: Timer finished");
                return Ok(Event::TimerTimedOut);
            }
        }

        // CHECK FOR BUTTON PRESS
        for button in Button::iterator() {
            for floor in elevator.requests.get_active_buttons(button) {
                if get::order_button(stream, button, floor).await? {
                    println!(
                        "Task {task_id}: Button {button:?} pressed at floor {floor}"
                    );
                    return Ok(Event::ButtonPress(button, floor));
                }
            }
        }

        // If no events are found, wait a tiny amount of time before checking for new requests
        sleep(Duration::from_millis(TIME_BETWEEN_EVENT_CHECKS)).await;
    }
}

async fn initialize(stream: &mut TcpStream) -> Result<Floor, std::io::Error> {
    if let Some(floor) = get::floor(stream).await? {
        send::floor_indicator(stream, floor).await?;
        return Ok(floor);
    }
    send::motor_direction(stream, Direction::Down).await?;
    loop {
        if let Some(floor) = get::floor(stream).await? {
            send::stop(stream).await?;
            send::floor_indicator(stream, floor).await?;
            return Ok(floor);
        }
    }
}
