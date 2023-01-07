use std::collections::HashMap;

use actix::{Actor, StreamHandler};
use actix_web::web::Payload;
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

use crate::actor::Game;

struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

type ActiveGames = HashMap<String, Game>;

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}
