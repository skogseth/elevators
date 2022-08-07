use std::io::prelude::*;
use std::net::TcpStream;

fn get_data(stream: &mut TcpStream, buffer: &mut [u8; 4]) -> std::io::Result<()> {
    stream.write(buffer)?;
    stream.read(buffer)?;
    Ok(())
}

pub fn order_button(stream: &mut TcpStream, button: usize, floor: usize) -> std::io::Result<[u8; 4]> {
    let mut buffer: [u8; 4] = [6, button as u8, floor as u8, 0];
    get_data(stream, &mut buffer)?;
    Ok(buffer)
}

pub fn floor(stream: &mut TcpStream) -> std::io::Result<[u8; 4]> {
    let mut buffer: [u8; 4] = [7, 0, 0, 0];
    get_data(stream, &mut buffer)?;
    Ok(buffer)
}

pub fn stop(stream: &mut TcpStream) -> std::io::Result<[u8; 4]> {
    let mut buffer: [u8; 4] = [8, 0, 0, 0];
    get_data(stream, &mut buffer)?;
    Ok(buffer)
}

pub fn obstruction_switch(stream: &mut TcpStream) -> std::io::Result<[u8; 4]> {
    let mut buffer: [u8; 4] = [9, 0, 0, 0];
    get_data(stream, &mut buffer)?;
    Ok(buffer)
}