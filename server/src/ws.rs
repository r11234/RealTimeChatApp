use std::sync::Mutex;
use actix_web::web::Data;
use crate::messages::WsMessage; 
use crate::lobby::Lobby; 
use actix::{Actor, Running, StreamHandler};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use uuid::Uuid;
use rand::Rng;

pub struct WsConnection {
    lobby_mtx: Data<Mutex<Lobby>>,
    id: Uuid,
    colour_hue: u16,
    name: String
} 

impl WsConnection {
    pub fn new(lobby: Data<Mutex<Lobby>>) -> WsConnection {  
        WsConnection {
            id: Uuid::new_v4(),
            lobby_mtx: lobby,
            colour_hue: rand::thread_rng().gen_range(0..360),
            name: Uuid::new_v4()
                .to_string()
                .split_once('-')
                .unwrap().0.to_string()
        }
    }
}

impl Actor for WsConnection {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address().recipient();

        addr.do_send(WsMessage {
            sender_colour_hue: self.colour_hue,
            sender_name: self.name.to_owned(),
            msg: None
        });

        self.lobby_mtx
            .lock()
            .unwrap().add_client(self.id, addr);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::prelude::Running {
        self.lobby_mtx.lock().unwrap().remove_client(self.id);
        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConnection {
    fn handle(
        &mut self, 
        msg: Result<ws::Message, ws::ProtocolError>, 
        _ctx: &mut Self::Context
    ) {
        match msg {
            Ok(Text(s)) => 
                self
                    .lobby_mtx
                    .lock()
                    .unwrap()
                    .broadcast_message(
                        s.to_string().as_str(),
                        self.colour_hue,
                        self.name.as_str()
                    ),
            Err(e) => panic!("{}", e),
            _ => ()
        }    
    }
}

impl Handler<WsMessage> for WsConnection {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        match serde_json::to_string(&msg) {
            Ok(v) => ctx.text(v),
            Err(e) => println!("Error while parsing: {e:?}")
        }
    }
}
