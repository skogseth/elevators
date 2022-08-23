use tokio::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use tokio::time::sleep;
use tokio::net::TcpStream;

use interface::types::{Button, Direction, Floor};
use interface::{get, send};

use crate::error::ElevatorError;
use crate::types::{Elevator, Message};

mod handle;
pub mod types;

use self::types::{Event, State};

const TIME_BETWEEN_EVENT_CHECKS: u64 = 10; // in milliseconds

pub async fn run(
    task_id: usize,
    mut stream: TcpStream,
    (tx, mut rx): (Sender<Message>, Receiver<Message>),
) -> Result<(), ElevatorError> {
    let start_floor = initialize(&mut stream).await.map_err(|e| {
        eprintln!("Could not start up elevator: {e}");
        ElevatorError {
            floor: Floor::from(0),
            state: State::Idle,
            critical: true,
        }
    })?;
    let mut elevator = Elevator::new(start_floor);

    loop {
        let event = wait_for_event(task_id, &mut stream, (&tx, &mut rx), &elevator).await;

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

async fn wait_for_event(
    task_id: usize,
    stream: &mut TcpStream,
    (tx, rx): (&Sender<Message>, &mut Receiver<Message>),
    elevator: &Elevator,
) -> Event {
    println!(
        "Task {task_id}: Waiting for event... (State::{:?}))",
        elevator.state
    );

    let msg = Message::ElevatorInfo {
        task_id,
        floor: elevator.floor,
        state: elevator.state,
        n_requests: elevator.requests.number_of_requests(),
    };
    tx.send(msg).await.unwrap();

    loop {
        // CHECK FOR FLOOR ARRIVAL
        if let Ok(opt_floor) = get::floor(stream).await {
            if let Some(floor) = opt_floor {
                if floor != elevator.floor {
                    println!("task {task_id}: Arrival at floor {floor}");
                    return Event::ArriveAtFloor(floor);
                }
            }
        } else {
            eprintln!("caught error in get::floor()!");
        }

        // CHECK FOR TIMER
        if let Some(timer) = elevator.timer {
            if timer.is_done() {
                eprintln!("task {task_id}: Timer finished");
                return Event::TimerTimedOut;
            }
        }

        // CHECK FOR MESSAGES
        if let Ok(msg) = rx.try_recv() {
            eprintln!("task {task_id}: Message received: {:?}", msg);
            return Event::MessageReceived(msg);
        }

        // CHECK FOR BUTTON PRESS
        for button in Button::iterator() {
            let floors = elevator.requests.get_active_buttons(button);

            for floor in floors {
                if let Ok(pressed) = get::order_button(stream, button, floor).await {
                    if pressed {
                        println!(
                            "task {task_id}: Button {:?} was pressed at floor {}",
                            button, floor
                        );
                        return Event::ButtonPress(button, floor);
                    }
                } else {
                    let identifier = format!("floor {floor} & button {button:?}");
                    eprintln!("caught error in get::order_button() for {identifier}");
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
