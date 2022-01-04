use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::thread;
use crate::utils::bytes2string;
use crate::mqtt_client::MQTTClient;

mod mqtt_client;
mod utils;

fn get_web_page() -> String{
    let mut page_html = String::new();
    let first_part = fs::read_to_string("src/firstPart.html")
        .expect("Something went wrong reading the file");
    page_html.push_str(&first_part);
    page_html.push_str("19");
    let second_part = fs::read_to_string("src/secondPart.html")
        .expect("Something went wrong reading the file");
    page_html.push_str(&second_part);
    return page_html;
}

fn main() {
    println!("Starting MQTT Conection");
    let client = MQTTClient::new("src/config.txt");
    thread::spawn(move || client.run());
    println!("Starting Web Server");
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    loop {
        let connection = listener.accept().unwrap();
        let mut client_stream: TcpStream = connection.0;
        thread::spawn(move || handle_client(&mut client_stream));
    }
}

fn handle_client(client_stream: &mut TcpStream) {
    let mut buff = [0u8; 1500];
    match client_stream.read(&mut buff) {
        Ok(_) => {
            let string_buff = bytes2string(&buff);
            let first_split = string_buff.split("\n");
            let first_line = first_split.collect::<Vec<&str>>()[0];
            let second_split = first_line.split(" ");
            let request_vec = second_split.collect::<Vec<&str>>();
            if !request_vec[0].eq("GET") {
                send_method_not_allowed(client_stream);
                return;
            }
            if !request_vec[1].eq("/") {
                send_not_found(client_stream);
                return;
            }
            send_page(client_stream);
            println!("La pagina fue enviada al solicitante");
        }
        Err(_) => {
            println!("Error al leer solicitud");
        }
    }
}

fn send_page(client_stream: &mut TcpStream) {
    let web_page = get_web_page();
    let mut request_data = String::new();
    request_data.push_str("HTTP/1.1 200 OK");
    request_data.push_str("\r\n");
    request_data.push_str("Cache-Control: no-cache");
    request_data.push_str("\r\n");
    request_data.push_str("Server: rustownserver");
    request_data.push_str("\r\n");
    request_data.push_str("Content-Type: text/html");
    request_data.push_str("\r\n");
    request_data.push_str("Content-Length: ");
    request_data.push_str(&web_page.len().to_string());
    request_data.push_str("\r\n");
    request_data.push_str("Connection: Closed");
    request_data.push_str("\r\n");
    request_data.push_str("\r\n");

    request_data.push_str(&web_page);

    client_stream.write_all(request_data.as_bytes()).unwrap();
}

fn send_method_not_allowed(client_stream: &mut TcpStream) {
    let mut request_data = String::new();
    request_data.push_str("HTTP/1.1 405 Method Not Allowed");
    request_data.push_str("\r\n");
    request_data.push_str("Content-Type: text/html");
    request_data.push_str("\r\n");
    request_data.push_str("Allow: GET");
    request_data.push_str("\r\n");
    request_data.push_str("\r\n");
    request_data.push_str("<h1>405 Only GET available!</h1>");
    client_stream.write_all(request_data.as_bytes()).unwrap();
}

fn send_not_found(client_stream: &mut TcpStream) {
    let mut request_data = String::new();
    request_data.push_str("HTTP/1.1 404 Not Found");
    request_data.push_str("\r\n");
    request_data.push_str("Content-Type: text/html");
    request_data.push_str("\r\n");
    request_data.push_str("\r\n");
    request_data.push_str("<h1>404 Not found</h1>");
    client_stream.write_all(request_data.as_bytes()).unwrap();
}
