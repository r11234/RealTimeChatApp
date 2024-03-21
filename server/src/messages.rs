use actix::prelude::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub msg: Option<String>,
    pub sender_colour_hue: u16,
    pub sender_name: String
}
