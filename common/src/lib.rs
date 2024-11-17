use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
    Welcome {version: u8},
    Subscribe {name: String},
    SubscribeResult(Result<(), SubscribeError>),
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