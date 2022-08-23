#![allow(dead_code)]
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;

use interface::types::Floor;

mod error;
mod state_machine;
mod types;

use crate::types::{Message, TaskInfo};

pub struct Config {
    pub n_elevators: usize,
    pub n_floors: usize,
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Executing run function");

    let Config {
        n_elevators,
        n_floors,
    } = config;
    println!("Number of elevator: {n_elevators}");
    println!("Number of floors: {n_floors}");

    Floor::initialize(n_floors);

    let mut tasks = Vec::new();
    let mut handles = Vec::new();

    let (tx_task, mut rx) = mpsc::channel(100);

    const HOST: [u8; 4] = [127, 0, 0, 1];
    const BASE_PORT: u16 = 10000;

    for i in 0..n_elevators {
        let addr = SocketAddr::from((HOST, BASE_PORT + i as u16));
        let stream = TcpStream::connect(addr).await?;
        println!("task {i} connected to {addr}");
        let (tx, rx_task) = mpsc::channel(100);
        let tx_task = tx_task.clone();

        let handle = tokio::spawn(async move {
            if let Err(_) = state_machine::run(i, stream, (tx_task, rx_task)).await {
                println!("Error occured in state machine");
            }
        });

        handles.push(handle);
        tasks.push(TaskInfo::new(i, tx));
    }

    loop {
        // CHECK IF ALL ELEVATORS ARE DOWN
        if tasks.is_empty() {
            break;
        }

        // TALK WITH ELEVATOR taskS
        match rx.try_recv() {
            Ok(msg) => match msg {
                Message::Request { floor, direction } => {
                    let mut tx = &tasks[0].transmitter;
                    let mut min_cost = std::usize::MAX;
                    for task in tasks.iter() {
                        let cost = task.cost_function(floor, direction);
                        if cost < min_cost {
                            tx = &task.transmitter;
                            min_cost = cost;
                        }
                    }
                    tx.send(msg).await.unwrap();
                }
                Message::HallButtonLight { .. } => {
                    // Send message to all elevators for hall button light
                    for task in tasks.iter() {
                        task.transmitter.send(msg).await.unwrap();
                    }
                }
                Message::ElevatorInfo {
                    task_id,
                    floor,
                    state,
                    n_requests,
                } => {
                    for task in tasks.iter_mut() {
                        if task.id == task_id {
                            task.floor = floor;
                            task.state = state;
                            task.n_requests = n_requests;
                            break;
                        }
                    }
                }
                Message::Shutdown => {
                    eprint!("Received shutdown message from task");
                }
            },
            Err(e) => {
                if e == TryRecvError::Disconnected {
                    break;
                }
            }
        }
    }

    // CHECK FOR ELEVATOR CRASHES
    for handle in handles {
        handle.await.expect("Task panicked, caught by main!")
    }

    Ok(())
}
