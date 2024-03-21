mod ws;
mod lobby;
use std::{collections::HashMap, sync::Mutex};

use lobby::Lobby;
mod messages;

use actix_web::{App, HttpServer, get, web::Data, web::Payload, Error, HttpResponse, HttpRequest};
use config::Config;

use crate::ws::WsConnection;

#[get("/")]
pub async fn start_connection_route(
    req: HttpRequest,
    stream: Payload,
    srv: Data<Mutex<Lobby>>,
) -> Result<HttpResponse, Error> {
    let ws = WsConnection::new(
        srv,
    );

    let resp = actix_web_actors::ws::start(ws, &req, stream)?;
    Ok(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Data::new(Mutex::new(Lobby::default())); 

    let settings = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build()
        .unwrap()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();


    HttpServer::new(move || {
        App::new()
            .service(start_connection_route) 
            .app_data(Data::clone(&chat_server))
    })
    .bind(format!("{}:{}", settings.get("url").unwrap(), settings.get("port").unwrap()))?
    .run()
    .await
}
