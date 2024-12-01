use std::env;
use std::net::TcpStream;
use common::client_args::{ CommandArgument, CommandArgumentsList };
use common::payloads::{ Direction, Payload, RegistrationError, ServerPayload, SubscribePlayerResult };

mod payloads_utils;
mod radar_view_utils;

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


fn get_tcp_stream(server_address_with_port: &str) -> TcpStream  {
    let tcp_stream: TcpStream = TcpStream::connect(&server_address_with_port).unwrap();
    tcp_stream
}


/**
 * send RegisterTeam payload to server and handle response
 */
fn register_team(team_name: &str, server_address_with_port: &str) {
    let mut stream = get_tcp_stream(&server_address_with_port);

    let register_team_payload = Payload::RegisterTeam { name: team_name.to_string() };

    payloads_utils::send_payload_to_server(&mut stream, &register_team_payload);

    match payloads_utils::receive_payload_from_server(&mut stream) {
        ServerPayload::RegisterTeamResult(Ok(register_team_response)) => {
            println!("Inscription réussie !");
            println!("Nombre de joueurs attendus : {}", register_team_response.expected_players);
            println!("Token d'inscription : {}", register_team_response.registration_token);

            subscribe(server_address_with_port, "player1", &register_team_response.registration_token);
        }
        ServerPayload::RegisterTeamResult(Err(registration_error)) => {
            match registration_error {
                RegistrationError::AlreadyRegistered => println!("Team already registered"),
                RegistrationError::InvalidName => println!("Invalid name for team."),
                RegistrationError::InvalidRegistrationToken => println!("Invalid registration token."),
                RegistrationError::TooManyPlayers => println!("Too many players.")
            }
        }
        _ => println!("Response not handled yet.")
    }
}

/**
 * send a SubscribePlayer payload to server, handle response
 * then handle RadarView server payload reception
 */
fn subscribe(server_address_with_port: &str, player_name: &str, registration_token: &str) {
    let mut stream = get_tcp_stream(&server_address_with_port);

    let subscribe_player_payload = Payload::SubscribePlayer {
        name: player_name.to_string(),
        registration_token : registration_token.to_string()
    };
    
    payloads_utils::send_payload_to_server(&mut stream, &subscribe_player_payload);

    // receive  player subscription confirmation
    match payloads_utils::receive_payload_from_server(&mut stream) {
        ServerPayload::SubscribePlayerResult(SubscribePlayerResult::Ok) => {
            println!("Player subscribtion succeed !");
        }
        ServerPayload::SubscribePlayerResult(SubscribePlayerResult::Err(registration_error)) => {
            match registration_error {
                RegistrationError::AlreadyRegistered => println!("Team already registered"),
                RegistrationError::InvalidName => println!("Invalid name for team."),
                RegistrationError::InvalidRegistrationToken => println!("Invalid registration token."),
                RegistrationError::TooManyPlayers => println!("Too many players.")
            }
        }
        _ => println!("Response not handled yet.")
    }

    // receive radar view
    match payloads_utils::receive_payload_from_server(&mut stream) {
        ServerPayload::RadarView(radar_view) => {
            println!("Received radar view : {}", radar_view);
        }
        _ => println!("Response not handled yet.")
    }

    //move_player(&mut stream, Direction::Left);
    
}


fn move_player(stream: &mut TcpStream, direction: Direction) {
    let move_player_payload = Payload::move_to(direction);

    payloads_utils::send_payload_to_server(stream, &move_player_payload);

    match payloads_utils::receive_payload_from_server(stream) {
        ServerPayload::ActionError(action_error) => match action_error {
            common::payloads::ActionError::CannotPassThroughWall => println!("CannotPassThroughWall"),
            common::payloads::ActionError::NoRunningChallenge => println!("NoRunningChallenge"),
            common::payloads::ActionError::SolveChallengeFirst => println!("SolveChallengeFirst"),
            common::payloads::ActionError::InvalidChallengeSolution => println!("InvalidChallengeSolution"),
        }
        _ => println!("Response not handled yet.")
    }
}