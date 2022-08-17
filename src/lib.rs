#![allow(dead_code)]
use std::cmp::Ordering;
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::thread::{self, JoinHandle};

mod elevator;
mod error;
mod message;
mod network;
mod state_machine;

use elevator::state::direction::Direction;

use crate::elevator::state::State;
use crate::error::ElevatorError;
use crate::message::Message;

pub struct Config {
    pub n_elevators: usize,
    pub n_floors: usize,
}

struct ThreadInfo {
    id: usize,
    handle: JoinHandle<Result<(), ElevatorError>>,
    transmitter: Sender<Message>,
    floor: usize,
    state: State,
    n_requests: usize,
}

impl ThreadInfo {
    fn new(
        id: usize,
        handle: JoinHandle<Result<(), ElevatorError>>,
        transmitter: Sender<Message>,
    ) -> Self {
        ThreadInfo {
            id,
            handle,
            transmitter,
            floor: 0,
            state: State::Idle,
            n_requests: 0,
        }
    }

    fn cost_function(&self, floor: usize, direction: Direction) -> usize {
        let in_direction = match self.state {
            State::Idle => true,
            State::Moving(dir) => direction == dir,
            State::Still(dir) => direction == dir,
        };
        let floor_difference = match &floor.cmp(&self.floor) {
            Ordering::Greater => floor - self.floor,
            Ordering::Equal => 0,
            Ordering::Less => self.floor - floor,
        };
        Self::cost_function_helper(self.state, floor_difference, self.n_requests, in_direction)
    }

    fn cost_function_helper(
        state: State,
        floor_difference: usize,
        n_requests: usize,
        in_direction: bool,
    ) -> usize {
        let state_value = match state {
            State::Idle => 0,
            State::Moving(..) => 1,
            State::Still(..) => 3,
        };
        state_value + (floor_difference) + 2 * (n_requests) + (!in_direction as usize)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Executing run function");

    let Config {
        n_elevators,
        n_floors,
    } = config;
    println!("Number of elevator: {n_elevators}");
    println!("Number of floors: {n_floors}");

    let mut threads = Vec::new();
    let (tx_thread, rx) = mpsc::channel();

    const HOST: [u8; 4] = [127, 0, 0, 1];
    const BASE_PORT: u16 = 10000;

    for i in 0..n_elevators {
        let addr = SocketAddr::from((HOST, BASE_PORT + i as u16));
        let stream = TcpStream::connect(addr)?;
        println!("Thread {i} connected to {addr}");
        let (tx, rx_thread) = mpsc::channel();
        let tx_thread = tx_thread.clone();

        let handle = thread::spawn(move || -> Result<(), ElevatorError> {
            state_machine::run(i, stream, (tx_thread, rx_thread), n_floors)
        });

        threads.push(ThreadInfo::new(i, handle, tx));
    }

    loop {
        // CHECK FOR ELEVATOR CRASHES
        for i in 0..threads.len() {
            if threads[i].handle.is_finished() {
                let thread = threads.remove(i);
                thread
                    .handle
                    .join()
                    .expect("Thread panicked, caught by main!")
                    .expect("Elevator crashed");
            }
        }

        // CHECK IF ALL ELEVATORS ARE DOWN
        if threads.is_empty() {
            break;
        }

        // TALK WITH ELEVATOR THREADS
        match rx.try_recv() {
            Ok(msg) => match msg {
                Message::Request { floor, direction } => {
                    // TODO: Implement algorithm to find best elevator
                    let mut tx = &threads[0].transmitter;
                    let mut min_cost = std::usize::MAX;
                    for thread in threads.iter() {
                        let cost = thread.cost_function(floor, direction);
                        if cost < min_cost {
                            tx = &thread.transmitter;
                            min_cost = cost;
                        }
                    }
                    tx.send(msg).unwrap();
                }
                Message::HallButtonLight { .. } => {
                    // Send message to all elevators for hall button light
                    for thread in threads.iter() {
                        thread.transmitter.send(msg).unwrap();
                    }
                }
                Message::ElevatorInfo {
                    thread_id,
                    floor,
                    state,
                    n_requests,
                } => {
                    for thread in threads.iter_mut() {
                        if thread.id == thread_id {
                            thread.floor = floor;
                            thread.state = state;
                            thread.n_requests = n_requests;
                            break;
                        }
                    }
                }
                Message::Shutdown => {
                    eprint!("Received shutdown message from thread");
                }
            },
            Err(e) => {
                if e == TryRecvError::Disconnected {
                    break;
                }
            }
        }
    }

    Ok(())
}
