use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn get_web_page() -> String{
    let mut page_html = String::new();
    page_html.push_str("<html><head><title>Hola manola</title>");
    page_html.push_str("<link href=\"https://unpkg.com/tailwindcss@^1.0/dist/tailwind.min.css\" rel=\"stylesheet\"></head>");
    page_html.push_str("<body class=\"bg-yellow-400\"><div class=\"text-center w-1/3 h-80 bg-white mt-10\">Hola gente linda, es mi pagina rust</div></body></html>");
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
