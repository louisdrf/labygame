use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    RegisterTeam { name: String },
    SubscribePlayer { name: String, registration_token: String },
    Action(Action),
}

impl Payload {
    pub fn move_to(direction: Direction) -> Self {
        Payload::Action(Action::MoveTo(direction))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    MoveTo(Direction)
}


#[derive(Serialize, Deserialize, Debug)]
pub enum ServerPayload {
    RegisterTeamResult(Result<RegisterTeamOk, RegistrationError>),
    SubscribePlayerResult(SubscribePlayerResult),
    ActionError(ActionError),
    RadarView(String)
}


#[derive(Serialize, Deserialize, Debug)]
pub enum RegisterTeamResult { 
    Ok { 
        expected_players: u8, 
        registration_token: String 
    }, 
    Err(RegistrationError) 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterTeamOk {
    pub expected_players: u8, 
    pub registration_token: String
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RegistrationError {
    AlreadyRegistered,
    InvalidName,
    InvalidRegistrationToken, 
    TooManyPlayers
}


#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribePlayerResult { 
    Ok, 
    Err(RegistrationError) 
}


#[derive(Serialize, Deserialize, Debug)]
pub enum ActionError {
    CannotPassThroughWall, 
    NoRunningChallenge, 
    SolveChallengeFirst, 
    InvalidChallengeSolution
}


#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom
}

impl Direction {
    pub fn to_string(&self) -> &str {
        match self {
            Direction::Left   => "Left",
            Direction::Right  => "Right",
            Direction::Top    => "Top",
            Direction::Bottom => "Bottom"
        }
    }
}




