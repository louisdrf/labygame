use std::env;
use std::net::TcpStream;
use std::io::{Write, Read};
use common::{Response, SubscribeError};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut address = String::from("127.0.0.1");
    let mut port = String::from("8080");

    for arg in &args[1..] {
        if let Some((key, value)) = parse_command_option(arg) {
            match key.as_str() {
                "port" => port = value,
                "address" => address = value,
                _ => {
                    eprintln!("Argument inconnu : {}", key);
                    std::process::exit(1);
                }
            }
        } else {
            eprintln!("Argument mal formaté : {}", arg);
            std::process::exit(1);
        }
    }

    let server_address_with_port = format!("{}:{}", address, port);
    launch_tcp_stream(&server_address_with_port);
}


fn launch_tcp_stream(server_address_with_port: &str) {

    match TcpStream::connect(&server_address_with_port) {
        Ok(mut tcp_stream) => {
            println!("Connecté à {}", server_address_with_port);
            subscribe(&mut tcp_stream);
        }
        Err(e) => {
            eprintln!("Erreur : impossible de se connecter à {}: {}", server_address_with_port, e);
        }
    }
}

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
            eprintln!("Invalid name !")
        },
        Ok(Response::SubscribeResult(Err(SubscribeError::AlreadyRegistered))) => {
            eprintln!("Name already registered !")
        },
        Err(_) => println!("Error while reading the subscribe response"),
        _ => eprintln!("Wrong answer")
    };
}



fn parse_command_option(arg: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = arg.splitn(2, '=').collect();

    if parts.len() == 2 {
        let key = parts[0];
        let value = parts[1];

        if key.starts_with("--") {
            let key = key.trim_start_matches("--").to_string();
            let value = value.to_string();

            return Some((key, value));
        }
    }

    None
}