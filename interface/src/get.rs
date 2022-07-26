use std::io::{Error, ErrorKind, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::types::{Button, Floor};

async fn get_data(stream: &mut TcpStream, buffer: &mut [u8; 4]) -> Result<()> {
    stream.write_all(buffer).await?;
    *buffer = [0; 4];
    stream.read(buffer).await?;
    Ok(())
}

pub async fn order_button(stream: &mut TcpStream, button: Button, floor: Floor) -> Result<bool> {
    let mut buffer: [u8; 4] = [6, u8::from(button), u8::from(floor), 0];
    get_data(stream, &mut buffer).await?;

    if buffer[0] != 6 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "incorrect feedback for buffer[0] in floor()",
        ));
    }

    let pressed = match buffer[1] {
        0 => false,
        1 => true,
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "incorrect feedback for buffer[1] in order_button()",
            ))
        }
    };

    Ok(pressed)
}

pub async fn floor(stream: &mut TcpStream) -> Result<Option<Floor>> {
    let mut buffer: [u8; 4] = [7, 0, 0, 0];
    get_data(stream, &mut buffer).await?;

    if buffer[0] != 7 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "incorrect feedback for buffer[0] in floor()",
        ));
    }

    let floor = match buffer[1] {
        0 => None,
        1 => Floor::from_value(buffer[2] as usize),
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "incorrect feedback for buffer[1] in floor()",
            ))
        }
    };

    Ok(floor)
}

pub async fn stop(stream: &mut TcpStream) -> Result<bool> {
    let mut buffer: [u8; 4] = [8, 0, 0, 0];
    get_data(stream, &mut buffer).await?;

    if buffer[0] != 8 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "incorrect feedback for buffer[0] in stop()",
        ));
    }

    let pressed = match buffer[1] {
        0 => false,
        1 => true,
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "incorrect feedback for buffer[1] in stop()",
            ))
        }
    };

    Ok(pressed)
}

pub async fn obstruction_switch(stream: &mut TcpStream) -> Result<bool> {
    let mut buffer: [u8; 4] = [9, 0, 0, 0];
    get_data(stream, &mut buffer).await?;

    if buffer[0] != 9 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "incorrect feedback for buffer[0] in obstruction_switch()",
        ));
    }

    let active = match buffer[1] {
        0 => false,
        1 => true,
        _ => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "incorrect feedback for buffer[1] in stop()",
            ))
        }
    };

    Ok(active)
}
