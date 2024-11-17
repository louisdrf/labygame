use std::env;
use std::net::TcpStream;
use std::io::{Write, Read};
use common::{Response, SubscribeError, CommandArgument, CommandArgumentsList};


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

    let mut address = String::from("127.0.0.1");
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
            subscribe(&mut tcp_stream);
        }
        Err(e) => {
            eprintln!("Error : connection to {} failed. Error: {}", server_address_with_port, e);
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


