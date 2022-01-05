use crate::mqtt_client::MqttClient;
use crate::page_manager::{send_method_not_allowed, send_not_found, send_page};
use crate::utils::bytes2string;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

mod mqtt_client;
mod page_manager;
mod utils;

fn main() {
    let temperature: i32 = 0;
    let lock_temperature = Arc::new(Mutex::new(temperature));
    let lock_cloned = lock_temperature.clone();
    println!("Starting MQTT Conection");
    let client = MqttClient::new("src/config.txt");
    thread::spawn(move || client.run(lock_cloned));
    println!("Starting Web Server");
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    loop {
        let connection = listener.accept().unwrap();
        let mut client_stream: TcpStream = connection.0;
        let new_lock_cloned = lock_temperature.clone();
        thread::spawn(move || handle_client(&mut client_stream, new_lock_cloned));
    }
}

fn handle_client(client_stream: &mut TcpStream, lock: Arc<Mutex<i32>>) {
    let mut buff = [0u8; 1500];
    match client_stream.read(&mut buff) {
        Ok(_) => {
            let string_buff = bytes2string(&buff);
            let first_split = string_buff.split('\n');
            let first_line = first_split.collect::<Vec<&str>>()[0];
            let second_split = first_line.split(' ');
            let request_vec = second_split.collect::<Vec<&str>>();
            if !request_vec[0].eq("GET") {
                send_method_not_allowed(client_stream);
                return;
            }
            if !request_vec[1].eq("/") {
                send_not_found(client_stream);
                return;
            }
            send_page(client_stream, lock);
            println!("La pagina fue enviada al solicitante");
        }
        Err(_) => {
            println!("Error al leer solicitud");
        }
    }
}
