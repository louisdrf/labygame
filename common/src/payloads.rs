use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    RegisterTeam { name: String },
    SubscribePlayer { name: String, registration_token: String },
    Action(Action),
}

impl Payload {
    pub fn move_to(direction: RelativeDirection) -> Self {
        Payload::Action(Action::MoveTo(direction))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    MoveTo(RelativeDirection)
}


#[derive(Serialize, Deserialize, Debug)]
pub enum ServerPayload {
    RegisterTeamResult(Result<RegisterTeamOk, RegistrationError>),
    SubscribePlayerResult(SubscribePlayerResult),
    ActionError(ActionError),
    RadarView(String),
    Hint(Hint)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Hint {
    RelativeCompass { angle: f32 },
    GridSize { columns: u32, rows: u32 },
    Secret(u64),
    SOSHelper
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


#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub enum RelativeDirection {
    Left,
    Right,
    Back,
    Front
}

impl RelativeDirection {
    pub fn to_string(&self) -> &str {
        match self {
            RelativeDirection::Left   => "Left",
            RelativeDirection::Right  => "Right",
            RelativeDirection::Back   => "Back",
            RelativeDirection::Front  => "Front"
        }
    }
}




