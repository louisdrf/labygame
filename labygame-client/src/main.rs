use std::io::{Write, Read};
use std::net::TcpStream;
use common::{Response, SubscribeError};

fn subscribe(stream: &mut TcpStream) {
    // Request
    let request = Response::Subscribe {name: String::from("Player1")};
    let serialized = serde_json::to_vec(&request).unwrap();
    stream.write_all(&serialized).unwrap();

    // Response
    let mut buffer = vec![0; 128];
    let bytes_read = stream.read(&mut buffer).unwrap();
    buffer.truncate(bytes_read);

    match serde_json::from_slice(&buffer) {
        Ok(Response::SubscribeResult(Ok(()))) => {
            println!("Success Subscribe !");
        },
        Ok(Response::SubscribeResult(Err(SubscribeError::InvalidName))) => {
            println!("Invalid name !")
        },
        Ok(Response::SubscribeResult(Err(SubscribeError::AlreadyRegistered))) => {
            println!("Name already registered !")
        },
        Err(_) => println!("Error while reading the subscribe response"),
        _ => println!("Wrong answer")
    };
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    subscribe(&mut stream);
}
