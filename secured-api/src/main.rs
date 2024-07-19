use actix::prelude::*;
use actix_cors::Cors;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use actix_web_middleware_keycloak_auth::{AlwaysReturnPolicy, DecodingKey, KeycloakAuth};
use utils::{app_state::AppState, db_pool::establish_connection, log::logging_setup};

use services::intervention_service::{
    create_intervention, get_intervention_with_comments, get_interventions_with_comments,
};
use web_socket_logic::web_socket::{ws_handler, NotificationServer};

mod controllers;
mod models;
mod repository;
mod routes;
mod schema;
mod services;
mod utils;
mod web_socket_logic;

/// The main entry point of the Actix-web application
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging for the application
    logging_setup();

    // Establish a connection pool to the database
    let pool = establish_connection();
    let state = AppState { conn: pool };

    // Fetch the Keycloak public key from environment variables
    let keycloak_pk =
        std::env::var("KEYCLOAK_PUBLIC_KEY").expect("KEYCLOAK_PUBLIC_KEY not found in .env file");
    let keycloak_pk =
        format!("-----BEGIN PUBLIC KEY-----\n{keycloak_pk}\n-----END PUBLIC KEY-----");

    // print the keycloak_pk
    println!("Keycloak public key: {:?}", keycloak_pk);

    let decoding_kc_public_key = match DecodingKey::from_rsa_pem(keycloak_pk.as_bytes()) {
        Ok(key) => key,
        Err(e) => {
            eprintln!("Invalid Keycloak public key format: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid Keycloak public key format",
            ));
        }
    };

    // Initialize the notification server
    let notification_server = NotificationServer::new(Data::new(state.clone())).start();

    let sentry_dsn = std::env::var("SENTRY_DSN").expect("SENTRY_DSN not found in .env file");
    let _guard = sentry::init((
        sentry_dsn,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    println!("Backend launched!");

    // Set up and run the HTTP server
    HttpServer::new(move || {
        // Configure CORS settings
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        // Configure Keycloak authentication middleware
        let keycloak_auth = KeycloakAuth {
            detailed_responses: true,
            passthrough_policy: AlwaysReturnPolicy,
            keycloak_oid_public_key: decoding_kc_public_key.clone(),
            required_roles: vec![],
        };

        // Set up the Actix-web application
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .app_data(web::Data::new(notification_server.clone()))
            .service(
                web::scope("/api/v1")
                    .wrap(keycloak_auth)
                    .route("/ws/{user_id}", web::get().to(ws_handler)),
            )
            .service(
                web::scope("/api/intervention")
                    .route("", web::post().to(create_intervention))
                    .route("", web::get().to(get_interventions_with_comments))
                    .route(
                        "/{interv_id}",
                        web::get().to(get_intervention_with_comments),
                    ),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
