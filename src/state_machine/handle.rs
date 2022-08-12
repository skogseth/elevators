use crate::elevator::Elevator;
use crate::elevator::state::State;
use crate::elevator::event::button::Button;
use crate::network::{get, send};

use std::net::TcpStream;

pub fn arrive_at_floor(stream: &mut TcpStream, elevator: &mut Elevator, floor: usize) {
    if let State::Moving(dir) = elevator.state {
        elevator.floor = floor;
        elevator.state = State::Still(dir);
        
        send::stop(stream);
        
        //check for request => stop and wait
        /*
        if elevator.check_for_request(floor) {
            elevator.set_state(State::Still);
        }
        */
    } else {
        eprintln!("arrived at floor {:?} without moving, state: {:?}", floor, elevator.state);
        // TODO? return Err(elevator.error(true));
    }
}

pub fn button_press(elevator: &mut Elevator, button: Button, floor: usize) {

}

pub fn timer_timed_out(elevator: &mut Elevator) {

}