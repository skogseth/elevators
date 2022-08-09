use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;

use crate::elevator::button::Button;

fn get_data(stream: &mut TcpStream, buffer: &mut [u8; 4]) -> Result<()> {
    stream.write(buffer)?;
    *buffer = [0; 4];
    stream.read(buffer)?;
    Ok(())
}

pub fn order_button(stream: &mut TcpStream, button: Button, floor: usize) -> Result<bool> {
    let mut buffer: [u8; 4] = [6, button as u8, floor as u8, 0];
    get_data(stream, &mut buffer)?;

    if buffer[0] != 6 {
        return Err(Error::new(ErrorKind::InvalidData, "incorrect feedback for buffer[0] in floor()"));
    }

    let pressed = match buffer[1] {
        0 => false,
        1 => true,
        _ => return Err(Error::new(ErrorKind::InvalidData, "incorrect feedback for buffer[1] in order_button()")),
    };

    Ok(pressed)
}

/*
pub fn buttons(stream: &mut TcpStream, n_floors: usize) -> Result<Vec<(usize, Button)>> {
    let mut buttons = Vec::new();

    for floor in 0..n_floors {
        for button in Button::iterator() {
            // TODO: Now returns error if ANY button/floor combo fails...
            if order_button(stream, button, floor)? {
                buttons.push((floor, button));
            }
        }
    }

    Ok(buttons)
}
*/

pub fn floor(stream: &mut TcpStream) -> Result<Option<usize>> {
    let mut buffer: [u8; 4] = [7, 0, 0, 0];
    get_data(stream, &mut buffer)?;

    if buffer[0] != 7 {
        return Err(Error::new(ErrorKind::InvalidData, "incorrect feedback for buffer[0] in floor()"));
    }

    let floor = match buffer[1] {
        0 => None,
        1 => Some(buffer[2] as usize),
        _ => return Err(Error::new(ErrorKind::InvalidData, "incorrect feedback for buffer[1] in floor()")),
    };

    Ok(floor)
}

pub fn stop(stream: &mut TcpStream) -> Result<bool> {
    let mut buffer: [u8; 4] = [8, 0, 0, 0];
    get_data(stream, &mut buffer)?;

    if buffer[0] != 8 {
        return Err(Error::new(ErrorKind::InvalidData, "incorrect feedback for buffer[0] in stop()"));
    }

    let pressed = match buffer[1] {
        0 => false,
        1 => true,
        _ => return Err(Error::new(ErrorKind::InvalidData, "incorrect feedback for buffer[1] in stop()")),
    };

    Ok(pressed)
}

pub fn obstruction_switch(stream: &mut TcpStream) -> Result<bool> {
    let mut buffer: [u8; 4] = [9, 0, 0, 0];
    get_data(stream, &mut buffer)?;

    if buffer[0] != 9 {
        return Err(Error::new(ErrorKind::InvalidData, "incorrect feedback for buffer[0] in obstruction_switch()"));
    }

    let active = match buffer[1] {
        0 => false,
        1 => true,
        _ => return Err(Error::new(ErrorKind::InvalidData, "incorrect feedback for buffer[1] in stop()")),
    };

    Ok(active)
}