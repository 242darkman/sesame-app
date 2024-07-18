use crate::models::zone::{NewZone, UpdateZone, Zone};
use crate::schema::zone::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;
/**
 * Céatio
 */
pub async fn create_zone(
    state: web::Data<AppState>,
    new_zone: web::Json<NewZone>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_zone = NewZone {
        name: new_zone.name.clone(),
        idlocation: new_zone.idlocation.clone(),
        numlevel: new_zone.numlevel.clone(),
    };

    match diesel::insert_into(zone)
        .values(&new_zone)
        .get_result::<Zone>(&mut conn)
    {
        Ok(inserted_zone) => HttpResponse::Created().json(inserted_zone),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert zone: {}", err))
        }
    }
}

pub async fn update_zone(
    state: web::Data<AppState>,
    id_zone: web::Path<String>,
    updated_zone: web::Json<UpdateZone>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let zone_uuid = match Uuid::parse_str(&id_zone) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format."),
    };

    let target = zone.filter(id.eq(zone_uuid));

    match diesel::update(target)
        .set((
            name.eq(&updated_zone.name),
            idlocation.eq(updated_zone.idlocation),
            numlevel.eq(updated_zone.numlevel),
        ))
        .execute(&mut conn)
    {
        Ok(0) => HttpResponse::NotFound().body("zone not found."),
        Ok(_) => HttpResponse::Ok().body("zone updated successfully."),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update zone: {}", err))
        }
    }
}
