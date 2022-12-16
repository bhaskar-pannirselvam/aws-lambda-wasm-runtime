use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fmt;
use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize)]
pub struct UserMessage {
    pub userName: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: String,
    pub responseMsg: String,
}

fn main() {
    let mut input = String::new();
    let mut handle = io::stdin().lock();
    handle.read_to_string(&mut input);

    let usrMsg: UserMessage = serde_json::from_str(&input).unwrap();

    let returnMsg = format!(
        "Hello, {}! You sent us the following message {}",
        usrMsg.userName, usrMsg.message
    );
    let sts = "Success";
    let resp = Response {
        status: String::from(sts),
        responseMsg: String::from(returnMsg),
    };
    let respJson = serde_json::to_string(&resp).unwrap();
    io::stdout().write(respJson.as_bytes()).unwrap();
}
