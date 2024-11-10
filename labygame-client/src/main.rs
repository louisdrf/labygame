use std::io::{Write, Read};
use std::net::TcpStream;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
enum Response {
    Welcome {version: u8}
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    let message = "Hello\n";
    stream.write_all(message.as_bytes()).unwrap();

    let mut response = String::new();
    stream.read_to_string(&mut response).unwrap();
    let deserialized: Response = serde_json::from_str(&response).unwrap();

    println!("Server response: {:?}", deserialized);
}
