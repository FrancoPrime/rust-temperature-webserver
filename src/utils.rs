use std::io::Read;
use std::net::TcpStream;

const MAX_MULTIPLIER: usize = 128 * 128 * 128;

pub fn remaining_length_read(stream: &mut TcpStream) -> Result<usize, String> {
    let mut buffer = [0u8; 1];
    if let Err(e) = stream.read_exact(&mut buffer) {
        return Err(format!("Error al leer del stream 1: {}", e.to_string()));
    }

    let mut byte: u8 = buffer[0];

    let mut multiplier: usize = 0x80;
    let mut value: usize = (&byte & 0x7F) as usize;

    while byte & 0x80 == 0x80 {
        if let Err(e) = stream.read_exact(&mut buffer) {
            return Err(format!("Error al leer del stream: {}", e.to_string()));
        }
        byte = buffer[0];
        value += ((byte & 0x7F) as usize) * &multiplier;
        multiplier *= 0x80;
        if multiplier > MAX_MULTIPLIER {
            return Err("Malformed reamining length".to_string());
        }
    }

    Ok(value)
}

pub fn bytes2string(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(str) => str.to_owned(),
        Err(_) => "".to_owned(),
    }
}