use std::net::UdpSocket;
use std::thread::sleep;
use std::time::{Duration, Instant};
use thiserror::Error;

// Milliseconds
const bit_time: u64 = 100;

#[derive(Error, Debug)]
pub enum LantennaError {
    #[error("Couldn't bind to socket")]
    SocketError(#[from] std::io::Error),

    #[error("Sending failed")]
    SendError(String),

    #[error("Couldn't encode the data")]
    EncodingError,
}

pub fn init(host: &str) -> Result<UdpSocket, LantennaError> {
    let socket = match UdpSocket::bind(host) {
        Ok(socket) => Ok(socket),
        Err(e) => Err(LantennaError::SocketError(e)),
    };

    socket
}

pub fn send(socket: &UdpSocket, receiver: &str, data: Vec<u8>) -> Result<(), LantennaError> {
    for byte in data {
        for i in 0..8 {
            let bit = {
                if byte & (1 << i) != 0 {
                    true
                } else {
                    false
                }
            };

            match bit {
                false => {
                    transmit_0(socket, receiver);
                }
                true => {
                    transmit_1(socket, receiver);
                }
            }
        }
    }
    Ok(())
}

fn transmit_0(socket: &UdpSocket, receiver: &str) -> Result<(), LantennaError> {
    let start = Instant::now();
    let half_bit_time = Duration::from_millis(bit_time / 2);

    while start.elapsed() < half_bit_time {
        socket.send_to(&[0x55; 1], receiver).expect("Couldn't send");
    }

    sleep(half_bit_time);

    Ok(())
}

fn transmit_1(socket: &UdpSocket, receiver: &str) -> Result<(), LantennaError> {
    let half_bit_time = Duration::from_millis(bit_time / 2);

    sleep(half_bit_time);

    let start = Instant::now();

    while start.elapsed() < half_bit_time {
        socket.send_to(&[0x55; 1], receiver).expect("Couldn't send");
    }

    Ok(())
}
