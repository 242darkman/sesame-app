use actix::prelude::*;
use actix_web::{web, HttpRequest, Responder};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self};
use std::time::{Duration, Instant};
use uuid::Uuid;

#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
struct Notification {
    user_id: Uuid,
    message: String,
}

#[derive(Message)]
#[rtype(result = "()")]
struct WsMessage(pub String);

struct WsSession {
    id: Uuid,
    user_id: Uuid,
    hb: Instant,
    addr: Addr<NotificationServer>,
    lifetime: Duration, // Ajout de la durée de vie maximale
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr.do_send(Connect {
            id: self.id,
            user_id: self.user_id,
            addr: ctx.address(),
        });

        // Commencez le timer pour la durée de vie maximale
        ctx.run_later(self.lifetime, |_actor, ctx| {
            ctx.stop();
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("WebSocket connection stopped for user_id: {}", self.user_id);
        self.addr.do_send(Disconnect {
            id: self.id,
            user_id: self.user_id,
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);
                let msg: Notification = serde_json::from_str(&text).unwrap();
                self.addr.do_send(msg);
                ctx.run_later(Duration::new(2, 0), |_, _| {
                    println!("2 seconds have passed since the first message was received.");
                });
            }
            _ => (),
        }
    }
}

impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

pub struct NotificationServer {
    sessions: HashMap<Uuid, Addr<WsSession>>,
    user_sessions: HashMap<Uuid, Vec<Uuid>>,
}

impl NotificationServer {
    pub fn new() -> NotificationServer {
        NotificationServer {
            sessions: HashMap::new(),
            user_sessions: HashMap::new(),
        }
    }
    fn send_friendship_notification(&self, user_id: Uuid, message: MessageType) {
        if let Some(sessions) = self.user_sessions.get(&user_id) {
            for session_id in sessions {
                if let Some(addr) = self.sessions.get(session_id) {
                    addr.do_send(WsMessage(message.to_string()));
                }
            }
        }
    }
}

impl Actor for NotificationServer {
    type Context = Context<Self>;
}

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub enum MessageType {
    RefreshFriendships,
    RefreshLists,
    RefreshSelectedList,
    OtherCase,
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageType::RefreshFriendships => write!(f, "RefreshFriendships"),
            MessageType::RefreshLists => write!(f, "RefreshLists"),
            MessageType::RefreshSelectedList => write!(f, "RefreshSelectedList"),
            MessageType::OtherCase => write!(f, "OtherCase"),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendFriendshipNotification {
    pub user_id: Uuid,
    pub message: MessageType,
}

impl Handler<SendFriendshipNotification> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: SendFriendshipNotification, _: &mut Context<Self>) {
        self.send_friendship_notification(msg.user_id, msg.message);
    }
}

struct Connect {
    id: Uuid,
    user_id: Uuid,
    addr: Addr<WsSession>,
}

impl Message for Connect {
    type Result = ();
}

impl Handler<Connect> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) {
        self.sessions.insert(msg.id, msg.addr.clone());
        self.user_sessions
            .entry(msg.user_id)
            .or_default()
            .push(msg.id);
    }
}

struct Disconnect {
    id: Uuid,
    user_id: Uuid,
}

impl Message for Disconnect {
    type Result = ();
}

impl Handler<Disconnect> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        self.sessions.remove(&msg.id);

        if let Some(sessions) = self.user_sessions.get_mut(&msg.user_id) {
            sessions.retain(|&x| x != msg.id);
            if sessions.is_empty() {
                self.user_sessions.remove(&msg.user_id);
            }
        }
    }
}

impl Handler<Notification> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: Notification, _: &mut Context<Self>) {
        if let Some(sessions) = self.user_sessions.get(&msg.user_id) {
            for session_id in sessions {
                if let Some(addr) = self.sessions.get(session_id) {
                    addr.do_send(WsMessage(serde_json::to_string(&msg).unwrap()));
                }
            }
        }
    }
}

pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<NotificationServer>>,
) -> impl Responder {
    let user_id = Uuid::parse_str(req.match_info().get("user_id").unwrap()).unwrap();
    let ws = WsSession {
        id: Uuid::new_v4(),
        user_id,
        hb: Instant::now(),
        addr: data.get_ref().clone(),
        lifetime: Duration::from_secs(86400), // 1 day
    };
    println!("Starting WS for user {}", user_id);
    ws::start(ws, &req, stream)
}
