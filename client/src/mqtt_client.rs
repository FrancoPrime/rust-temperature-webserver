use rand::Rng;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

static TEMPERATURE_TOPIC: &str = "RustTemperature";
static CLIENT_ID: &str = "oracle";
static DEFAULT_IP: &str = "127.0.0.1:1883";
static KEEP_ALIVE: u8 = 200;

pub struct MqttClient {
    full_ip: String,
}

impl MqttClient {
    pub fn new(file_path: &str) -> Self {
        let mut full_ip = DEFAULT_IP.to_owned();
        let file: String = match std::fs::read_to_string(file_path) {
            Ok(file) => file,
            Err(_) => "Error when trying to open the file".to_string(),
        };
        let lines = file.lines();
        for line in lines {
            let name_and_value: Vec<&str> = line.split('=').collect();
            let config_name: String = name_and_value[0]
                .to_lowercase()
                .replace(' ', "")
                .to_string();
            let value: String = name_and_value[1].replace(' ', "").to_string();
            if config_name.eq("full_ip") {
                full_ip = value;
            }
        }
        MqttClient { full_ip }
    }

    fn send_temperature(&self, stream: &mut TcpStream) {
        loop {
            let mut rng = rand::thread_rng();
            let temperature: i32 = rng.gen_range(-10..45);
            let mut temperature_str = temperature.to_string().as_bytes().to_vec();
            let mut buffer_publish: Vec<u8> = Vec::new();
            let topic_subscribed = TEMPERATURE_TOPIC.to_owned();
            let mut topic_subscribed_bytes: Vec<u8> = topic_subscribed.as_bytes().to_vec();
            buffer_publish.push(0x30); //Publish code
            buffer_publish.push((2 + topic_subscribed_bytes.len() + temperature_str.len()) as u8);
            buffer_publish.push(0);
            buffer_publish.push(topic_subscribed_bytes.len() as u8);
            buffer_publish.append(&mut topic_subscribed_bytes);
            buffer_publish.append(&mut temperature_str);

            stream.write_all(&buffer_publish).unwrap();
            println!("Envio una nueva temperatura: {}", temperature);
            sleep(Duration::from_secs(10));
        }
    }

    fn connect_and_wait_for_connack(&self, stream: &mut TcpStream) {
        let client_id = CLIENT_ID.to_owned();
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
        buffer.push(2);
        buffer.push(0);
        buffer.push(KEEP_ALIVE);
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

    pub fn send_pingreq_packet(stream: &mut TcpStream) {
        let buffer = [0xC0, 0_u8];
        stream.write_all(&buffer).unwrap();
    }

    pub fn run(&self) {
        let mut stream = TcpStream::connect(&self.full_ip).unwrap();
        self.connect_and_wait_for_connack(&mut stream);
        println!("Connected to MQTT Server");
        let mut pingreq_stream = stream.try_clone().expect("Cannot clone stream");
        thread::spawn(move || loop {
            MqttClient::send_pingreq_packet(&mut pingreq_stream);
            sleep(Duration::from_secs(KEEP_ALIVE as u64));
        });
        self.send_temperature(&mut stream);
    }
}
