use std::io::prelude::*;
use std::net::TcpStream;

use crate::types::{Button, Direction, Floor};

fn send_data(stream: &mut TcpStream, buffer: &[u8; 4]) -> std::io::Result<()> {
    stream.write(buffer)?;
    Ok(())
}

pub fn reload_config(stream: &mut TcpStream) -> std::io::Result<()> {
    let buffer: [u8; 4] = [0, 0, 0, 0];
    send_data(stream, &buffer)
}

pub fn motor_direction(stream: &mut TcpStream, direction: Direction) -> std::io::Result<()> {
    let buffer: [u8; 4] = [1, u8::from(direction), 0, 0];
    send_data(stream, &buffer)
}

pub fn stop(stream: &mut TcpStream) -> std::io::Result<()> {
    let buffer: [u8; 4] = [1, 0, 0, 0];
    send_data(stream, &buffer)
}

pub fn order_button_light(stream: &mut TcpStream, button: Button, floor: Floor, on: bool) -> std::io::Result<()> {
    let value = if on { 1 } else { 0 };
    let buffer: [u8; 4] = [2, u8::from(button), u8::from(floor), value];
    send_data(stream, &buffer)
}

pub fn floor_indicator(stream: &mut TcpStream, floor: Floor) -> std::io::Result<()> {
    let buffer: [u8; 4] = [3, u8::from(floor), 0, 0];
    send_data(stream, &buffer)
}

pub fn door_open_light(stream: &mut TcpStream, on: bool) -> std::io::Result<()> {
    let buffer: [u8; 4] = [4, u8::from(on), 0, 0];
    send_data(stream, &buffer)
}

pub fn stop_button_light(stream: &mut TcpStream, on: bool) -> std::io::Result<()> {
    let buffer: [u8; 4] = [5, u8::from(on), 0, 0];
    send_data(stream, &buffer)
}