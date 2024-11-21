/*use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use common::Payload;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = vec![0; 128];
    let bytes_read = stream.read(&mut buffer).unwrap();
    buffer.truncate(bytes_read);

    let request: Payload = serde_json::from_slice(&buffer).unwrap();
    if let Payload::SubscribePlayer { name } = request {
        println!("Received subscription request from: {}", name);
        let payload = Payload::SubscribePlayerResult(Ok(()));
        let serialized = serde_json::to_vec(&payload).unwrap();
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
    */