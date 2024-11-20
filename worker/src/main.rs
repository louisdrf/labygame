use std::env;
use std::net::TcpStream;
use std::io::{Write, Read};
use common::{
    Payload, 
    SubscribePlayerError, 
    CommandArgument, 
    CommandArgumentsList};


/**
 * param @arg command argument as "--arg_name=value"
 * @returns Option<CommandArgument> with parsed argument name (--arg_name) and value
 */
fn parse_command_argument(arg: &str) -> Option<CommandArgument> {
    let command_name_and_value: Vec<&str> = arg.splitn(2, '=').collect();

    if command_name_and_value.len() == 2 {
        let arg_name = command_name_and_value[0];
        let arg_value = command_name_and_value[1];

        if arg_name.starts_with("--") {
            let arg_name = arg_name.to_string();
            let arg_value = arg_value.to_string();

            return Some(CommandArgument { name: arg_name, value: arg_value });
        }
    }

    None
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut address = String::from("localhost");
    let mut port = String::from("8778");

    for arg in &args[1..] {

        if let Some(command_argument) = parse_command_argument(arg) {

            let command_name: &str = command_argument.get_name_as_str();

            match CommandArgumentsList::from_command_name(command_name) {
                Some(CommandArgumentsList::Port)    => port = command_argument.value,
                Some(CommandArgumentsList::Address) => address = command_argument.value,
                Some(CommandArgumentsList::GroupName) => {
                    eprintln!("Argument not handled yet : {}", command_argument.name);
                    std::process::exit(1);
                }
                None => {
                    eprintln!("Unknown argument name : {}", command_argument.name);
                    std::process::exit(1);
                }
            }
        } 
        else {
            eprintln!("Bad format for argument : {}", arg);
            std::process::exit(1);
        }
    }

    let server_address_with_port = format!("{}:{}", address, port);

    launch_tcp_stream(&server_address_with_port);
}


fn launch_tcp_stream(server_address_with_port: &str) {

    match TcpStream::connect(&server_address_with_port) {
        Ok(mut tcp_stream) => {
            println!("Connected to {}", server_address_with_port);
            say_hello(&mut tcp_stream);
            subscribe(&mut tcp_stream);
        }
        Err(e) => {
            eprintln!("Error : connection to {} failed. Error: {}", server_address_with_port, e);
        }
    }
}


fn say_hello(stream: &mut TcpStream) {
    let request = Payload::Hello;
    let message: Vec<u8> = to_tcp_message(&request);

    match stream.write_all(&message) {
        Ok(()) => {
            println!("Message écrit avec succès dans le writer.");
        }
        Err(e) => {
            eprintln!("Erreur lors de l'écriture dans le writer : {}", e);
        }
    }
}

fn subscribe(stream: &mut TcpStream) {
    let request = Payload::SubscribePlayer {name: String::from("Player1")};
    let message: Vec<u8> = to_tcp_message(&request);

    match stream.write_all(&message) {
        Ok(()) => {
            println!("Message written succesfully in the writer.");
        }
        Err(e) => {
            eprintln!("Error while writing in the writer : {}", e);
        }
    }

    let mut buffer = vec![0; 128];
    match stream.read(&mut buffer) {
        Ok(0) => {
            // 0 octet lu : la connexion est fermée proprement
            println!("Server closed connection.");
        }
        Ok(bytes_read) => {
            println!("Received message : {} bytes.", bytes_read);
            buffer.truncate(bytes_read); 
        }
        Err(e) => {
            eprintln!("Error while reading : {}", e);
        }
    };

    match serde_json::from_slice(&buffer) {
        Ok(Payload::SubscribePlayerResult(Ok(()))) => {
            println!("Success SubscribePlayer !");
        },
        Ok(Payload::SubscribePlayerResult(Err(SubscribePlayerError::InvalidName))) => {
            eprintln!("Invalid name !")
        },
        Ok(Payload::SubscribePlayerResult(Err(SubscribePlayerError::AlreadyRegistered))) => {
            eprintln!("Name already registered !")
        },
        Err(_) => println!("Error while reading the subscribe Payload"),
        _ => eprintln!("Wrong answer")
    };
}

fn to_tcp_message(payload: &Payload) -> Vec<u8> {
    let serialized = serde_json::to_vec(payload).unwrap();
    let message_size = serialized.len() as u32;

    println!("Serialized payload: {:?}", String::from_utf8_lossy(&serialized));
    println!("Payload size: {}", message_size);

    let mut message = Vec::new();
    message.extend(&message_size.to_le_bytes()); // ajouter la taille du message au payload
    message.extend(serialized);                  // ajouter les données serialisées

    message
}



