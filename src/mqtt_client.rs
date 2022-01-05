use crate::utils::{bytes2string, remaining_length_read};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

static TEMPERATURE_TOPIC: &str = "RustTemperature";

pub struct MqttClient {
    full_ip: String,
}

impl MqttClient {
    pub fn new(_file_path: &str) -> Self {
        MqttClient {
            full_ip: "127.0.0.1:1883".to_owned(),
        }
    }

    fn subscribe_to_temperature(&self, stream: &mut TcpStream) {
        let mut buffer_subscribe: Vec<u8> = Vec::new();
        let topic_subscribed = TEMPERATURE_TOPIC.to_owned();
        let mut topic_subscribed_bytes: Vec<u8> = topic_subscribed.as_bytes().to_vec();
        buffer_subscribe.push(0x80); //Subscribe code
        buffer_subscribe.push((5 + topic_subscribed_bytes.len()) as u8);
        buffer_subscribe.push(0);
        buffer_subscribe.push(57);
        buffer_subscribe.push(0);
        buffer_subscribe.push(topic_subscribed_bytes.len() as u8);
        buffer_subscribe.append(&mut topic_subscribed_bytes);
        buffer_subscribe.push(1);
        stream.write_all(&buffer_subscribe).unwrap();
    }

    fn connect_and_wait_for_connack(&self, stream: &mut TcpStream) {
        let client_id = "temperatureBroker".to_owned();
        let client_id_bytes = client_id.as_bytes();
        let mut buffer: Vec<u8> = Vec::with_capacity(14 + client_id_bytes.len());
        buffer.push(0x10); //Connect packet
        buffer.push((12 + client_id_bytes.len()) as u8); //Hardcoded length
        buffer.push(0);
        buffer.push(4);
        buffer.push(77); // M
        buffer.push(81); // Q
        buffer.push(84); // T
        buffer.push(84); // T
        buffer.push(4); // Protocol Level
        buffer.push(0);
        buffer.push(0);
        buffer.push(200);
        buffer.push(0);
        buffer.push(2);
        for byte in client_id_bytes.iter() {
            buffer.push(*byte);
        }
        stream.write_all(&buffer).unwrap();
        let mut can_go_on = false;
        while !can_go_on {
            let mut num_buffer = [0u8; 2]; //Recibimos 2 bytes
            match stream.read_exact(&mut num_buffer) {
                Ok(_) => {
                    let package_type = num_buffer[0];
                    if package_type != 0x20 {
                        panic!("Conection Error");
                    }
                    let mut buffer_paquete: Vec<u8> = vec![0; num_buffer[1] as usize];
                    stream.read_exact(&mut buffer_paquete).unwrap();
                    can_go_on = true;
                }
                Err(_) => {
                    println!("Error while trying to read from the stream");
                }
            }
        }
    }

    fn wait_for_publishes(&self, stream: &mut TcpStream, lock: Arc<Mutex<i32>>) {
        loop {
            let mut num_buffer = [0u8; 1];
            match stream.read_exact(&mut num_buffer) {
                Ok(_) => {
                    let package_type = num_buffer[0];
                    let buff_size = remaining_length_read(stream).unwrap();
                    let mut buffer_packet: Vec<u8> = vec![0; buff_size as usize];
                    stream.read_exact(&mut buffer_packet).unwrap();
                    if package_type & 0xF0 != 0x30 {
                        continue;
                    }
                    let topic_name_len: usize =
                        buffer_packet[1] as usize + ((buffer_packet[0] as usize) << 8) as usize;
                    let topic_name = bytes2string(&buffer_packet[2..(2 + &topic_name_len)]);
                    if topic_name.eq(TEMPERATURE_TOPIC) {
                        let content = bytes2string(
                            &buffer_packet[(4 + &topic_name_len)..buffer_packet.len()],
                        );
                        let new_temperature;
                        match content.parse::<i32>() {
                            Ok(val) => {
                                new_temperature = val;
                            }
                            Err(_) => {
                                println!("Error while reading");
                                new_temperature = 0;
                            }
                        }
                        match lock.lock() {
                            Ok(mut locked) => {
                                *locked = new_temperature;
                            }
                            Err(_) => {
                                println!("Error while trying to access the lock");
                            }
                        }
                    }
                }
                Err(_) => {
                    println!("Error while trying to read from the stream");
                }
            }
        }
    }

    pub fn run(&self, lock: Arc<Mutex<i32>>) {
        let mut stream = TcpStream::connect(&self.full_ip).unwrap();
        self.connect_and_wait_for_connack(&mut stream);
        self.subscribe_to_temperature(&mut stream);
        self.wait_for_publishes(&mut stream, lock);
    }
}
