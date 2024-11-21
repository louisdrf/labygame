use std::env;
use std::net::TcpStream;
use std::io::{Write, Read};
use common::{
    CommandArgument, CommandArgumentsList, Payload, RegisterTeamResult, RegistrationError, ServerPayload};
use serde_json::ser;


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
            register_team(&mut tcp_stream, "team_zoza");
            //subscribe(&mut tcp_stream);
        }
        Err(e) => {
            eprintln!("Error : connection to {} failed. Error: {}", server_address_with_port, e);
        }
    }
}

fn register_team(stream: &mut TcpStream, team_name: &str) {
    let register_team_message = Payload::RegisterTeam { 
        name: team_name.to_string() 
    };

    send_message_to_server(stream, &register_team_message);

    match receive_register_team_message_from_server(stream) {
        RegisterTeamResult::Ok {
            expected_players,
            registration_token,
        } => {
            println!("Inscription réussie !");
            println!("Nombre de joueurs attendus : {}", expected_players);
            println!("Token d'inscription : {}", registration_token);
        }
        
        RegisterTeamResult::Err(err) => {
            eprintln!("Erreur lors de la réception de la réponse : {:?}", err);
        }
    }
}


fn subscribe(stream: &mut TcpStream, registration_token: &str, player_name: &str) {
    let message = Payload::SubscribePlayer {
        name: player_name.to_string(),
        registration_token: registration_token.to_string()
    };
    
    send_message_to_server(stream, &message);
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

fn send_message_to_server(stream: &mut TcpStream, message: &Payload) {
    let encoded_message: Vec<u8> = to_tcp_message(&message);

    match stream.write_all(&encoded_message) {
        Ok(()) => {
            println!("Message written succesfully in the writer.");
        }
        Err(e) => {
            eprintln!("Error while writing in the writer : {}", e);
        }
    }
}

fn receive_register_team_message_from_server(stream: &mut TcpStream) -> RegisterTeamResult {
    let mut message_size_buffer = [0u8; 4];  // 4 octets pour la taille du message
    let _ = stream.read_exact(&mut message_size_buffer).unwrap();
    let message_size = u32::from_le_bytes(message_size_buffer) as usize;

    // Lire le payload JSON envoyé par le serveur
    let mut buffer = vec![0u8; message_size];
    let _ = stream.read_exact(&mut buffer).unwrap();

    // Désérialiser le JSON dans un Payload
    let server_response = serde_json::from_slice(&buffer).unwrap();

    server_response
}