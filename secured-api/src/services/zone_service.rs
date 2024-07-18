use crate::models::zone::Zone;
use crate::schema::zone::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

pub async fn get_zones(state: web::Data<AppState>) -> Result<Vec<Zone>, actix_web::Error> {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match zone.load::<Zone>(&mut conn) {
        Ok(all_zones) => Ok(all_zones),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Failed to retrieve zones: {}",
            err
        ))),
    }
}
