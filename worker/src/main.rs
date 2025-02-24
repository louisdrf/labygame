use std::time::Duration;
use std::{env, thread};
use std::net::TcpStream;
use common::client_args::{ CommandArgument, CommandArgumentsList };
use common::payloads::{ RelativeDirection, Payload, RegistrationError, ServerPayload, SubscribePlayerResult };
use radar_view_utils::RadarCell;

mod payloads_utils;
mod radar_view_utils;
mod hint_utils;

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

           // for i in 0..register_team_response.expected_players {
                subscribe(server_address_with_port, "toto", &register_team_response.registration_token);
           // }
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

fn select_best_move(view: &Vec<Vec<RadarCell>>, hint_direction: Option<RelativeDirection>) -> RelativeDirection {
    let center = 3;
    let left_cell  = view[center][center - 2].clone();
    let right_cell = view[center][center + 2].clone();
    let front_cell = view[center - 2][center].clone();
    let back_cell = view[center + 2][center].clone();

    // Check if objective `G` (GOAL) is visible and go directly to it
    for (i, row) in view.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == RadarCell::Exit {
                println!("-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*--*-*-*-*-*-*-*-*-*-*-*-*--*-*");
                println!("Goal detected at ({}, {})! Prioritizing movement...", i, j);
                println!("-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*--*-*-*-*-*-*-*-*-*-*-*-*--*-*");
                let x;
                if j > center {
                    x =  RelativeDirection::Right;
                } else if j < center {
                    x =  RelativeDirection::Left;
                } else if i < center {
                    x =  RelativeDirection::Front;
                } else {
                    x = RelativeDirection::Back;
                }
                let move_player_payload = Payload::move_to(x);
                println!("Exit reached! Stopping the game.");
                std::process::exit(0);
            }
        }
    }

    if let Some(hint_dir) = hint_direction {
        match hint_dir {
            RelativeDirection::Right    if right_cell == RadarCell::Open => return RelativeDirection::Right,
            RelativeDirection::Front    if front_cell == RadarCell::Open => return RelativeDirection::Front,
            RelativeDirection::Left     if left_cell == RadarCell::Open =>  return RelativeDirection::Left,
            RelativeDirection::Back     if back_cell == RadarCell::Open =>  return RelativeDirection::Back,
            _ => println!("⚠️ Hint direction is blocked, switching to fallback strategy."),
        }
    }

    // Classic strategy if no `G` (GOAL) found
    if      right_cell == RadarCell::Open   { return RelativeDirection::Right; } 
    else if front_cell == RadarCell::Open   { return RelativeDirection::Front; } 
    else if left_cell == RadarCell::Open    { return RelativeDirection::Left; } 
    else                                    { return RelativeDirection::Back; }
}

/**
 * send a SubscribePlayer payload to server, handle response
 * then handle RadarView server payload reception
 */
 /*
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
            let mut view = radar_view_utils::decode(&radar_view);
            let center = 3;

            let mut next_move: RelativeDirection;

            while view[center][center] != RadarCell::Exit {
                let left_cell = view[center][center - 2].clone();
                let right_cell = view[center][center + 2].clone();
                let front_cell = view[center - 2][center].clone();
            
                if      right_cell == RadarCell::Open { next_move = RelativeDirection::Right; }
                else if front_cell == RadarCell::Open { next_move = RelativeDirection::Front; }
                else if left_cell  == RadarCell::Open { next_move = RelativeDirection::Left; }
                else                                  { next_move = RelativeDirection::Back; }

                let move_player_payload = Payload::move_to(next_move);
                payloads_utils::send_payload_to_server(&mut stream, &move_player_payload);

                match payloads_utils::receive_payload_from_server(&mut stream) {
                    ServerPayload::RadarView(radar_view) => {
                        view = radar_view_utils::decode(&radar_view);
                    },
                    ServerPayload::ActionError(action_error) => match action_error {
                        common::payloads::ActionError::CannotPassThroughWall => {
                            println!("CannotPassThroughWall - Changing direction");
                            println!("current direction : {:?}", next_move);
                            println!("Received radar view: {}", radar_view);
                            break;
                        }
                        _ => println!("Action error: {:?}", action_error),
                    },
                    _ => println!("Response not handled yet."),
                }

                thread::sleep(Duration::from_millis(200));
            }
        }
        _ => println!("Response not handled yet.")
    }    
}
*/

fn subscribe(server_address_with_port: &str, player_name: &str, registration_token: &str) {
    let mut stream = get_tcp_stream(&server_address_with_port);
    let center = 3;
    let mut hint_direction: Option<RelativeDirection> = None;

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
    let mut view = loop {
        match payloads_utils::receive_payload_from_server(&mut stream) {
            ServerPayload::RadarView(radar_view) => {
                println!("Received radar view : {}", radar_view);
                break radar_view_utils::decode(&radar_view);
            },
            other => {
                println!("Ignoring non-RadarView message: {:?}", other);
                continue;
            },
        }
    };

    loop {
        if view[center][center] == RadarCell::Exit {
            println!("Exit reached! Stopping the game.");
            std::process::exit(0);
        }

        let next_move = select_best_move(&view, hint_direction);

        let move_player_payload = Payload::move_to(next_move);
        payloads_utils::send_payload_to_server(&mut stream, &move_player_payload);

        loop {
            match payloads_utils::receive_payload_from_server(&mut stream) {
                ServerPayload::RadarView(radar_view) => {
                    println!("Received radar view: {}", radar_view);
                    view = radar_view_utils::decode(&radar_view);
                    hint_direction = None;
                    break;
                },
                ServerPayload::Hint(hint) => {
                    match hint {
                        common::payloads::Hint::RelativeCompass { angle } => {
                            println!("RelativeCompass received! Angle to Exit: {}°", angle);
                            hint_direction = Some(hint_utils::direction_from_angle(angle));
                        }
                        common::payloads::Hint::GridSize { columns, rows } => {
                            println!("Grid size received: {}x{}", columns, rows);
                        }
                        common::payloads::Hint::Secret(secret) => {
                            println!("Secret received: {}", secret);
                        }
                        common::payloads::Hint::SOSHelper => {
                            println!("SOSHelper received! Possible emergency situation.");
                        }
                    }
                    continue; 
                },
                ServerPayload::ActionError(action_error) => {
                    match action_error {
                        common::payloads::ActionError::CannotPassThroughWall => {
                            println!("Cannot pass through wall! Changing direction...");
                            continue; 
                        }
                        _ => println!("Action error: {:?}", action_error),
                    }
                },
                other => {
                    println!("Unexpected message, ignoring: {:?}", other);
                },
            }
        }
    }
}

