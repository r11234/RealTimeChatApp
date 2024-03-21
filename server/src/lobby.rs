use crate::messages::WsMessage;
use actix::prelude::Recipient;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Lobby {
    sessions: HashMap<Uuid, Recipient<WsMessage>>,
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
        }
    }
}

impl Lobby {
    pub fn add_client(&mut self, id: Uuid, addr: Recipient<WsMessage>) {
        self.sessions.insert(id, addr);
    }

    pub fn remove_client(&mut self, id: Uuid) {
        self.sessions.remove(&id);
    }
    
    pub fn broadcast_message(
        &self, 
        message: &str, 
        colour_hue: u16, 
        name: &str
    ) {
        for client_id in self.sessions.keys() {
            
            if let Some(socket_recipient) = self.sessions.get(client_id) {
                socket_recipient
                    .do_send(WsMessage {
                        msg: Some(message.to_owned()),
                        sender_colour_hue: colour_hue,
                        sender_name: name.to_owned()
                    });
            }

        }
    }
}
