use std::io::prelude::*;
use std::net::TcpStream;

fn send_data(stream: &mut TcpStream, buffer: &[u8; 4]) -> std::io::Result<()> {
    stream.write(buffer)?;
    Ok(())
}

pub fn reload_config(stream: &mut TcpStream) -> std::io::Result<()> {
    let buffer: [u8; 4] = [0, 0, 0, 0];
    send_data(stream, &buffer)
}

pub fn motor_direction(stream: &mut TcpStream, direction: i32) -> std::io::Result<()> {
    let buffer: [u8; 4] = [1, direction as u8, 0, 0];
    send_data(stream, &buffer)
}

pub fn order_button_light(stream: &mut TcpStream, button: usize, floor: usize, on: bool) -> std::io::Result<()> {
    let value = if on { 1 } else { 0 };
    let buffer: [u8; 4] = [2, button as u8, floor as u8, value];
    send_data(stream, &buffer)
}

pub fn floor_indicator(stream: &mut TcpStream, floor: usize) -> std::io::Result<()> {
    let buffer: [u8; 4] = [3, floor as u8, 0, 0];
    send_data(stream, &buffer)
}

pub fn door_open_light(stream: &mut TcpStream, on: bool) -> std::io::Result<()> {
    let value = if on { 1 } else { 0 };
    let buffer: [u8; 4] = [4, value, 0, 0];
    send_data(stream, &buffer)
}