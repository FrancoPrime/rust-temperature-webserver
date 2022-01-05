use std::fs;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

fn get_web_page(temperature: i32) -> String {
    let mut page_html = String::new();
    let first_part =
        fs::read_to_string("src/firstPart.html").expect("Something went wrong reading the file");
    page_html.push_str(&first_part);
    page_html.push_str(&temperature.to_string());
    let second_part =
        fs::read_to_string("src/secondPart.html").expect("Something went wrong reading the file");
    page_html.push_str(&second_part);
    page_html
}

pub fn send_page(client_stream: &mut TcpStream, lock: Arc<Mutex<i32>>) {
    let temperature;
    match lock.lock() {
        Ok(locked) => {
            temperature = *locked;
        }
        Err(_) => {
            temperature = 0;
            println!("Error while reading the lock")
        }
    }
    let web_page = get_web_page(temperature);
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

pub fn send_method_not_allowed(client_stream: &mut TcpStream) {
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

pub fn send_not_found(client_stream: &mut TcpStream) {
    let mut request_data = String::new();
    request_data.push_str("HTTP/1.1 404 Not Found");
    request_data.push_str("\r\n");
    request_data.push_str("Content-Type: text/html");
    request_data.push_str("\r\n");
    request_data.push_str("\r\n");
    request_data.push_str("<h1>404 Not found</h1>");
    client_stream.write_all(request_data.as_bytes()).unwrap();
}
