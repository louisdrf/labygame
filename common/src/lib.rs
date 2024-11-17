use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Payload {
    Welcome {version: u8},
    Subscribe {name: String},
    SubscribeResult(Result<(), SubscribeError>),
    Action(Action),
    ActionResult(Result<(), ActionError>)
}

impl Payload {
    pub fn move_to(direction: Direction) -> Self {
        Payload::Action(Action::MoveTo(direction))
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionResult {
    Ok,
    Completed,
    Err(ActionError)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionError {
    InvalidMove
}

#[derive(Serialize, Deserialize, Debug)]

pub enum Action {
    MoveTo(Direction)
}

impl Action {
    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self)
    }
}






#[derive(Debug)]
pub struct CommandArgument {
    pub name: String,
    pub value: String,
}

impl CommandArgument {
    pub fn get_name_as_str(&self) -> &str {
        &self.name.as_str()
    }

    pub fn get_value_as_str(&self) -> &str {
        &self.value.as_str()
    }
}


pub enum CommandArgumentsList {
    Port,
    Address,
    GroupName
}

impl CommandArgumentsList {
    pub fn from_command_name(arg_name: &str) -> Option<Self> {
        match arg_name {
            "--address" => Some(CommandArgumentsList::Address),
            "--port"    => Some(CommandArgumentsList::Port),
            "--group"   => Some(CommandArgumentsList::GroupName),
            _           => None,
        }
    }
}