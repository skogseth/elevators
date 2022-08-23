use std::io::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::types::{Button, Direction, Floor};

async fn send_data(stream: &mut TcpStream, buffer: &[u8; 4]) -> Result<()> {
    stream.write_all(buffer).await?;
    Ok(())
}

pub async fn reload_config(stream: &mut TcpStream) -> Result<()> {
    let buffer: [u8; 4] = [0, 0, 0, 0];
    send_data(stream, &buffer).await
}

pub async fn motor_direction(stream: &mut TcpStream, direction: Direction) -> Result<()> {
    let buffer: [u8; 4] = [1, u8::from(direction), 0, 0];
    send_data(stream, &buffer).await
}

pub async fn stop(stream: &mut TcpStream) -> Result<()> {
    let buffer: [u8; 4] = [1, 0, 0, 0];
    send_data(stream, &buffer).await
}

pub async fn order_button_light(
    stream: &mut TcpStream,
    button: Button,
    floor: Floor,
    on: bool,
) -> Result<()> {
    let buffer: [u8; 4] = [2, u8::from(button), u8::from(floor), u8::from(on)];
    send_data(stream, &buffer).await
}

pub async fn floor_indicator(stream: &mut TcpStream, floor: Floor) -> Result<()> {
    let buffer: [u8; 4] = [3, u8::from(floor), 0, 0];
    send_data(stream, &buffer).await
}

pub async fn door_open_light(stream: &mut TcpStream, on: bool) -> Result<()> {
    let buffer: [u8; 4] = [4, u8::from(on), 0, 0];
    send_data(stream, &buffer).await
}

pub async fn stop_button_light(stream: &mut TcpStream, on: bool) -> Result<()> {
    let buffer: [u8; 4] = [5, u8::from(on), 0, 0];
    send_data(stream, &buffer).await
}
