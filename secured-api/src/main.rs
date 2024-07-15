use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_web_middleware_keycloak_auth::{AlwaysReturnPolicy, DecodingKey, KeycloakAuth};
use utils::{app_state::AppState, db_pool::establish_connection, log::logging_setup};

use self::routes::user_routes::init_user_routes;

mod controllers;
mod models;
mod repository;
mod routes;
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
            keycloak_oid_public_key: DecodingKey::from_rsa_pem(keycloak_pk.as_bytes()).unwrap(),
            required_roles: vec![],
        };

        // Set up the Actix-web application
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/api/v1")
                    .wrap(keycloak_auth)
                    .configure(init_user_routes),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
