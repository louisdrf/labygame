use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use common::Response;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = vec![0; 128];
    let bytes_read = stream.read(&mut buffer).unwrap();
    buffer.truncate(bytes_read);

    let request: Response = serde_json::from_slice(&buffer).unwrap();
    if let Response::Subscribe { name } = request {
        println!("Received subscription request from: {}", name);
        let response = Response::SubscribeResult(Ok(()));
        let serialized = serde_json::to_vec(&response).unwrap();
        stream.write_all(&serialized).unwrap();
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8778").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}