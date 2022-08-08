use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use std::net::TcpStream;

fn get_data(stream: &mut TcpStream, buffer: &mut [u8; 4]) -> Result<()> {
    stream.write(buffer)?;
    *buffer = [0; 4];
    stream.read(buffer)?;
    Ok(())
}

pub fn order_button(stream: &mut TcpStream, button: usize, floor: usize) -> Result<[u8; 4]> {
    let mut buffer: [u8; 4] = [6, button as u8, floor as u8, 0];
    get_data(stream, &mut buffer)?;
    Ok(buffer)
}

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

pub fn stop(stream: &mut TcpStream) -> Result<[u8; 4]> {
    let mut buffer: [u8; 4] = [8, 0, 0, 0];
    get_data(stream, &mut buffer)?;
    Ok(buffer)
}

pub fn obstruction_switch(stream: &mut TcpStream) -> Result<[u8; 4]> {
    let mut buffer: [u8; 4] = [9, 0, 0, 0];
    get_data(stream, &mut buffer)?;
    Ok(buffer)
}