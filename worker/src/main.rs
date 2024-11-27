use std::env;
use std::net::TcpStream;
use std::io::{Write, Read};
use common::{
    CommandArgument, CommandArgumentsList, Payload, RegistrationError, ServerPayload};


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
    let mut team_name: String = String::from("team1");

    for arg in &args[1..] {

        if let Some(command_argument) = parse_command_argument(arg) {

            let command_name: &str = command_argument.get_name_as_str();

            match CommandArgumentsList::from_command_name(command_name) {
                Some(CommandArgumentsList::Port)    => port = command_argument.value,
                Some(CommandArgumentsList::Address) => address = command_argument.value,
                Some(CommandArgumentsList::GroupName) => team_name = command_argument.value,
                None => {
                    eprintln!("Unknown argument name : {}", command_argument.name);
                    std::process::exit(1);
                }
            }
        } 
        else {
            eprintln!("Bad format for argument : {}. Expected : --arg=value", arg);
            std::process::exit(1);
        }
    }

    let server_address_with_port = format!("{}:{}", address, port);

    register_team(&team_name, &server_address_with_port);
}


fn get_tcp_stream(server_address_with_port: &str) -> TcpStream  {
    let tcp_stream: TcpStream = TcpStream::connect(&server_address_with_port).unwrap();
    tcp_stream
}


fn register_team(team_name: &str, server_address_with_port: &str) {
    let mut stream = get_tcp_stream(&server_address_with_port);

    let register_team_payload = Payload::RegisterTeam { name: team_name.to_string() };

    send_payload_to_server(&mut stream, &register_team_payload);

    match receive_payload_from_server(&mut stream) {
        ServerPayload::RegisterTeamResult(Ok(register_team_response)) => {
            println!("Inscription réussie !");
            println!("Nombre de joueurs attendus : {}", register_team_response.expected_players);
            println!("Token d'inscription : {}", register_team_response.registration_token);
        }
        ServerPayload::RegisterTeamResult(Err(registration_error)) => {
            match registration_error {
                RegistrationError::AlreadyRegistered => println!("Team already registered"),
                RegistrationError::InvalidName => println!("Invalid name for team.")
            }
        }
        _ => println!("Response not handled yet.")
    }
}


fn subscribe(server_address_with_port: &str, player_name: &str, registration_token: &str) {
    let mut stream = get_tcp_stream(&server_address_with_port);

    let subscribe_player_payload = Payload::SubscribePlayer {
        name: player_name.to_string(),
        registration_token : registration_token.to_string()
    };
    
    send_payload_to_server(&mut stream, &subscribe_player_payload);
}

fn to_tcp_payload(payload: &Payload) -> Vec<u8> {
    let serialized = serde_json::to_vec(payload).unwrap();
    let payload_size = serialized.len() as u32;

    println!("Serialized payload: {:?}", String::from_utf8_lossy(&serialized));
    println!("Payload size: {}", payload_size);

    let mut payload = Vec::new(); 
    payload.extend(&payload_size.to_le_bytes()); // ajouter la taille du payload au payload
    payload.extend(serialized);                  // ajouter les données serialisées

    payload
}

fn send_payload_to_server(stream: &mut TcpStream, payload: &Payload) {
    let encoded_payload: Vec<u8> = to_tcp_payload(&payload);

    match stream.write_all(&encoded_payload) {
        Ok(()) => {
            println!("payload written succesfully in the writer.");
        }
        Err(e) => {
            eprintln!("Error while writing in the writer : {}", e);
        }
    }
}

fn receive_payload_from_server(stream: &mut TcpStream) -> ServerPayload {
    
    let mut payload_size_buffer = [0u8; 4];
    stream.read_exact(&mut payload_size_buffer).unwrap();
    let payload_size = u32::from_le_bytes(payload_size_buffer) as usize;

    let mut buffer = vec![0u8; payload_size];
    stream.read_exact(&mut buffer).unwrap();

    let server_response: ServerPayload = serde_json::from_slice(&buffer).unwrap();

    server_response
}
 

