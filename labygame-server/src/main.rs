use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut message = String::new();

    match buf_reader.read_line(&mut message) {
        Ok(_) => {
            if message.trim() == "Hello" {
                let response = r#"{"Welcome":{"version":1}}"#;
                stream.write_all(response.as_bytes()).unwrap();
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