use crate::models::location::{Location, NewLocation, UpdateLocation};
use crate::schema::locations::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;

/**
 * Céatio
 */

pub async fn create_location(
    state: web::Data<AppState>,
    new_location: web::Json<NewLocation>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_location = NewLocation {
        name: new_location.name.clone(),
    };

    match diesel::insert_into(locations)
        .values(&new_location)
        .get_result::<Location>(&mut conn)
    {
        Ok(inserted_location) => HttpResponse::Created().json(inserted_location),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert location: {}", err))
        }
    }
}


/**
 * 
 * Création de location avec websocket
 */
/* pub async fn create_location(
    pool: &DbPool,
    new_location: NewLocation,
) -> Result<Location, String> {
    let mut conn = pool
        .get()
        .map_err(|_| "Failed to get a connection from the pool.".to_string())?;

    let new_location = NewLocation {
        name: new_location.name,
    };

    diesel::insert_into(locations::table)
        .values(&new_location)
        .get_result::<Location>(&mut conn)
        .map_err(|err| format!("Failed to insert location: {}", err))
} */


pub async fn update_location(
    state: web::Data<AppState>,
    id_location: web::Path<String>,
    updated_location: web::Json<UpdateLocation>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");
    let location_uuid = match Uuid::parse_str(&id_location) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format."),
    };

    let target = locations.filter(id.eq(location_uuid));
    match diesel::update(target)
        .set(name.eq(&updated_location.name))
        .execute(&mut conn)
    {
        Ok(0) => HttpResponse::NotFound().body("Location not found."),
        Ok(_) => HttpResponse::Ok().body("Location updated successfully."),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update location: {}", err))
        }
    }
}

pub async fn get_locations(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match locations.load::<Location>(&mut conn) {
        Ok(all_locations) => HttpResponse::Ok().json(all_locations),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to retrieve locations: {}", err)),
    }
}
