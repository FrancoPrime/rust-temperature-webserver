use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

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
    println!("Iniciando servidor web");
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    loop {
        let connection = listener.accept().unwrap();
        let mut client_stream: TcpStream = connection.0;
        let mut buff = [0u8; 1500];
        match client_stream.read(&mut buff) {
            Ok(_) => {
                println!("{}", bytes2string(&buff));
                let mut request_data = String::new();
                request_data.push_str("HTTP/1.1 200 OK");
                request_data.push_str("\r\n");
                request_data.push_str("Cache-Control: no-cache");
                request_data.push_str("\r\n");
                request_data.push_str("Server: rustownserver");
                request_data.push_str("\r\n");
                request_data.push_str("Content-Type: text/html");
                request_data.push_str("\r\n");
                request_data.push_str("Content-Type: text/html");
                request_data.push_str("\r\n");
                request_data.push_str("Connection: Closed");
                request_data.push_str("\r\n");
                request_data.push_str("\r\n");

                request_data.push_str(&get_web_page());

                client_stream.write_all(request_data.as_bytes()).unwrap();
                println!("Enviada la pagina");
            }
            Err(_) => {
                println!("Erorsito");
            }
        }
    }
}

pub fn bytes2string(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(str) => str.to_owned(),
        Err(_) => "".to_owned(),
    }
}
