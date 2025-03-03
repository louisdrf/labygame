use std::{io::{Read, Write}, net::TcpStream};
use common::payloads::{Payload, ServerPayload, ActionError};


pub fn to_tcp_payload(payload: &Payload) -> Vec<u8> {
    let serialized = serde_json::to_vec(payload).unwrap();
    let payload_size = serialized.len() as u32;

    println!("Serialized payload: {:?}", String::from_utf8_lossy(&serialized));
    println!("Payload size: {}", payload_size);

    let mut payload = Vec::new(); 
    payload.extend(&payload_size.to_le_bytes()); // ajouter la taille du payload au payload
    payload.extend(serialized);                  // ajouter les données serialisées

    payload
}

pub fn send_payload_to_server(stream: &mut TcpStream, payload: &Payload) {
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

pub fn receive_payload_from_server(stream: &mut TcpStream) -> ServerPayload {

    let mut payload_size_buffer = [0u8; 4];
    stream.read_exact(&mut payload_size_buffer).unwrap();
    let payload_size = u32::from_le_bytes(payload_size_buffer) as usize;

    let mut buffer = vec![0u8; payload_size];
    stream.read_exact(&mut buffer).unwrap();

    let server_response: ServerPayload = serde_json::from_slice(&buffer).unwrap();
    
    match &server_response {
        ServerPayload::RadarView(_) => {
            println!("New RadarView received.");
        },
        ServerPayload::Hint(hint) => {
            println!("Hint received: {:?}", hint);
        },
        ServerPayload::ActionError(error) => {
            match error {
                ActionError::CannotPassThroughWall => {
                    println!("Wall detected! Need to recalculate path.");
                }
                ActionError::NoRunningChallenge => {
                    println!("ERREUR : Aucun challenge en cours !");
                }
                ActionError::SolveChallengeFirst => {
                    println!("ERREUR : Il faut résoudre un challenge avant d'agir !");
                }
                ActionError::InvalidChallengeSolution => {
                    println!("ERREUR : La solution du challenge est incorrecte !");
                }
            }
        },
        _ => {
            println!("Received other server message: {:?}", server_response);
        }
    }

    server_response
}