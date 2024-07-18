use crate::models::defaults::Defaults;
use crate::schema::defaults::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

pub async fn get_problem_types(
    state: web::Data<AppState>,
) -> Result<Vec<Defaults>, actix_web::Error> {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match defaults.load::<Defaults>(&mut conn) {
        Ok(all_default_types) => Ok(all_default_types),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Failed to retrieve zones: {}",
            err
        ))),
    }
}
