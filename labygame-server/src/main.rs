use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use serde::Serialize;

#[derive(Serialize)]
enum Response {
    Welcome {version: u8}
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut message = String::new();

    match buf_reader.read_line(&mut message) {
        Ok(_) => {
            println!("Message received: {:?}", message);
            if message.trim() == "Hello" {
                let response = Response::Welcome { version: 1 };
                let serialized = serde_json::to_string(&response).unwrap();
                stream.write_all(serialized.as_bytes()).unwrap();
                println!("Sent 'Welcome' message");
            }
        }
        Err(_) => {}
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}