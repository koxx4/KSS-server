use actix::{Actor, AsyncContext, Handler, Message, StreamHandler, WrapFuture};
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use log::debug;
use mongodb::Client;
use crate::services::events_service::get_unread_count;

const GET_EVENTS_COUNT_CMD: &'static str = "GET_RECENT_EVENTS";

struct KssWebSocket {
    db_client: web::Data<Client>,
}

impl Actor for KssWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

struct UnreadEventsCount(u64);

impl Message for UnreadEventsCount {
    type Result = ();
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for KssWebSocket {

    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {

        if let Ok(ws::Message::Text(text)) = msg {
            debug!("KssWebSocket got incoming message: {}", text);
            self.handle_text(text.to_string(), ctx);
        }
    }
}


impl KssWebSocket {

    pub fn new(db_client: web::Data<Client>) -> Self {
        Self { db_client }
    }

    fn handle_text(&self, text: String, ctx: &mut ws::WebsocketContext<Self>) {

        if text == GET_EVENTS_COUNT_CMD {

            let addr = ctx.address();
            let db_client = self.db_client.clone();

            ctx.spawn(async move {
                let count = get_unread_count(db_client.get_ref()).await;
                addr.do_send(UnreadEventsCount(count));
            }.into_actor(self));
        }
    }
}

impl Handler<UnreadEventsCount> for KssWebSocket {
    type Result = ();

    fn handle(&mut self, msg: UnreadEventsCount, ctx: &mut Self::Context) {
        let message = format!("{}", msg.0);
        ctx.text(message);
    }
}


pub async fn ws_check_new(req: HttpRequest, stream: web::Payload, db_client: web::Data<Client>) -> Result<HttpResponse, Error> {
    debug!("Incoming on ws_check_new");
    let resp = ws::start(KssWebSocket::new(db_client), &req, stream);
    debug!("{:?}", resp);
    resp
}