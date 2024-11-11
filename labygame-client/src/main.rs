use std::io::{Write, Read};
use std::net::TcpStream;
use common::Response;

fn subscribe(stream: &mut TcpStream) {
    let request = Response::Subscribe {name: String::from("Player1")};
    let serialized = serde_json::to_vec(&request).unwrap();
    stream.write_all(&serialized).unwrap();

    let mut buffer = vec![0; 128];
    let bytes_read = stream.read(&mut buffer).unwrap();
    buffer.truncate(bytes_read);

    let response: Response = serde_json::from_slice(&buffer).unwrap();
    println!("Server response: {:?}", response);
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    subscribe(&mut stream);
}
