use actix::prelude::*;
use actix_web::{web, HttpRequest, Responder};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self};
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::models::defaults::ProblemTypesResponse;
use crate::models::location::LocationsResponse;
use crate::models::zone::ZonesResponse;
use crate::services::defaults_service::get_problem_types;
use crate::services::location_service::get_locations;
use crate::services::user_service::initialize_user;
use crate::services::zone_service::get_zones;
use crate::utils::app_state::AppState;

/// Notification structure used to send notification messages to WebSocket clients.
#[derive(Message, Serialize, Deserialize)]
#[rtype(result = "()")]
struct Notification {
    user_id: Uuid,
    message: String,
}

/// WebSocket message containing a string
#[derive(Message)]
#[rtype(result = "()")]
struct WsMessage(pub String);

/// Structure representing a WebSocket session
struct WsSession {
    id: Uuid,
    user_id: Uuid,
    hb: Instant,
    addr: Addr<NotificationServer>,
    lifetime: Duration, // Maximum lifetime of the session
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    /// Initialize the WebSocket session
    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr.do_send(Connect {
            id: self.id,
            user_id: self.user_id,
            addr: ctx.address(),
        });

        // Start the timer for the maximum lifetime
        ctx.run_later(self.lifetime, |_actor, ctx| {
            ctx.stop();
        });
    }

    /// Clean up when the WebSocket session is stopped
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
                println!("Received Ping");
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
                println!("Received Pong");
            }
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);
                match serde_json::from_str::<Notification>(&text) {
                    Ok(notification) => {
                        self.addr.do_send(notification);
                        ctx.run_later(Duration::new(2, 0), |_, _| {
                            println!("2 seconds have passed since the first message was received.");
                        });
                    }
                    Err(e) => match text.to_string().as_str() {
                        "REQUEST_LOCATIONS" => self.addr.do_send(RequestLocations {
                            addr: ctx.address(),
                        }),
                        "REQUEST_ZONES" => self.addr.do_send(RequestZones {
                            addr: ctx.address(),
                        }),
                        "REQUEST_PROBLEM_TYPES" => self.addr.do_send(RequestProblemTypes {
                            addr: ctx.address(),
                        }),
                        _ => {
                            println!("Failed to deserialize message: {:?}", e);
                            println!("Message content: {}", text);
                            ctx.stop();
                        }
                    },
                }
            }
            _ => {
                println!("Received other type of message or an error occurred.");
            }
        }
    }
}

impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// Notification server managing WebSocket sessions
pub struct NotificationServer {
    sessions: HashMap<Uuid, Addr<WsSession>>,
    user_sessions: HashMap<Uuid, Vec<Uuid>>,
    pool: web::Data<AppState>, // Database connection pool
}

impl NotificationServer {
    /// Create a new notification server
    pub fn new(pool: web::Data<AppState>) -> NotificationServer {
        NotificationServer {
            sessions: HashMap::new(),
            user_sessions: HashMap::new(),
            pool,
        }
    }
    /// Send a friendship notification to a specific user
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

/// Enumeration of message types that can be sent by the server
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

/// Message for connecting a WebSocket session
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

/// Message for disconnecting a WebSocket session
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

/// Message to request locations
#[derive(Message)]
#[rtype(result = "()")]
struct RequestLocations {
    addr: Addr<WsSession>,
}

impl Handler<RequestLocations> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: RequestLocations, _: &mut Context<Self>) {
        let pool = self.pool.clone();
        let addr = msg.addr.clone();
        actix::spawn(async move {
            match get_locations(pool).await {
                Ok(locations_result) => {
                    let response = LocationsResponse {
                        locations: locations_result,
                    };
                    let response_json = serde_json::to_string(&response)
                        .unwrap_or_else(|_| "{\"locations\":[]}".to_string());
                    println!("Sending locations: {}", response_json);
                    addr.do_send(WsMessage(response_json));
                }
                Err(_) => {
                    addr.do_send(WsMessage("{\"locations\":[]}".to_string()));
                }
            }
        });
    }
}

/// Message to request zones
#[derive(Message)]
#[rtype(result = "()")]
struct RequestZones {
    addr: Addr<WsSession>,
}

impl Handler<RequestZones> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: RequestZones, _: &mut Context<Self>) {
        let pool = self.pool.clone();
        let addr = msg.addr.clone();
        actix::spawn(async move {
            match get_zones(pool).await {
                Ok(zones_result) => {
                    let response = ZonesResponse {
                        zones: zones_result,
                    };
                    let response_json = serde_json::to_string(&response)
                        .unwrap_or_else(|_| "{\"zones\":[]}".to_string());
                    println!("Sending zones: {}", response_json);
                    addr.do_send(WsMessage(response_json));
                }
                Err(_) => {
                    addr.do_send(WsMessage("{\"zones\":[]}".to_string()));
                }
            }
        });
    }
}

/// Message to request probem types
#[derive(Message)]
#[rtype(result = "()")]
struct RequestProblemTypes {
    addr: Addr<WsSession>,
}

impl Handler<RequestProblemTypes> for NotificationServer {
    type Result = ();

    fn handle(&mut self, msg: RequestProblemTypes, _: &mut Context<Self>) {
        let pool = self.pool.clone();
        let addr = msg.addr.clone();
        actix::spawn(async move {
            match get_problem_types(pool).await {
                Ok(problem_types_result) => {
                    let response = ProblemTypesResponse {
                        problem_types: problem_types_result,
                    };
                    let response_json = serde_json::to_string(&response)
                        .unwrap_or_else(|_| "{\"problem_types\":[]}".to_string());
                    println!("Sending problem types: {}", response_json);
                    addr.do_send(WsMessage(response_json));
                }
                Err(_) => {
                    addr.do_send(WsMessage("{\"problem_types\":[]}".to_string()));
                }
            }
        });
    }
}

/// Handler for WebSocket connections
pub async fn ws_handler(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Addr<NotificationServer>>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let user_id = Uuid::parse_str(req.match_info().get("user_id").unwrap()).unwrap();

    // Initialize the user
    if let Err(e) = initialize_user(user_id, pool).await {
        println!("Failed to initialize user: {:?}", e);
    }

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
