use std::fmt::Display;

pub trait Error {
    fn to_string(&self)->String;
}

#[derive(Debug)]
pub struct BotError{
    msg:String
}

impl Error for BotError {
    fn to_string(&self)->String {
        self.msg.clone()
    }
}

impl Display for BotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.msg)
    }
}

impl From<tungstenite::Error> for BotError {
    fn from(value: tungstenite::Error) -> Self {
        Self{msg:value.to_string()}
    }
}

impl From<std::string::String> for BotError {
    fn from(value: std::string::String) -> Self {
        Self { msg: value }
    }
}

impl From<serde_json::Error> for BotError {
    fn from(value: serde_json::Error) -> Self {
        Self { msg: value.to_string() }
    }
}

impl From<tungstenite::http::uri::InvalidUri> for BotError {
    fn from(value: tungstenite::http::uri::InvalidUri) -> Self {
        Self { msg: value.to_string() }
    }
}

