use std::env;
use std::net::TcpStream;
use std::io::{Write, Read};
use common::{Response, SubscribeError};

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_port = "7878"; 

    match args.len() {
        3 => {
            let server_address = &args[1];
            let port = &args[2];
            println!("{}:{}", server_address, port);
            let address_with_port = format!("{}:{}", server_address, port); 
            println!("Connexion à l'adresse : {}...", address_with_port);
            launch_tcp_stream(&address_with_port);  
        },
        2 => {
            let server_address = &args[1];
            let address_with_port = format!("{}:{}", server_address, default_port); 
            println!("Connexion à l'adresse : {}... (port par défaut)", address_with_port);
            launch_tcp_stream(&address_with_port); 
        },
        1 => {
            eprintln!("Erreur : aucune adresse spécifiée.");
            eprintln!("Usage: worker [server_address]");
            std::process::exit(1);
        },
        _ => {
            eprintln!("Erreur : trop d'arguments spécifiés.");
            eprintln!("Usage: worker [server_address]");
            std::process::exit(1);
        }
    }
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

