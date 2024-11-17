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