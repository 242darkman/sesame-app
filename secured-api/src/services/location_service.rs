use crate::models::location::Location;
use crate::schema::locations::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

pub async fn get_locations(state: web::Data<AppState>) -> Result<Vec<Location>, actix_web::Error> {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match locations.load::<Location>(&mut conn) {
        Ok(all_locations) => Ok(all_locations),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Failed to retrieve locations: {}",
            err
        ))),
    }
}
