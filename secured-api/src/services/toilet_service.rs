use crate::models::toilet::{NewToilet, Toilet, UpdateToilet};
use crate::schema::toilet::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;
/**
 * Céatio
 */
pub async fn create_toilet(
    state: web::Data<AppState>,
    new_toilet: web::Json<NewToilet>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_toilet = NewToilet {
        toiletstatus: new_toilet.toiletstatus.clone(),
        idzone: new_toilet.idzone.clone(),
    };

    match diesel::insert_into(toilet)
        .values(&new_toilet)
        .get_result::<Toilet>(&mut conn)
    {
        Ok(inserted_toilet) => HttpResponse::Created().json(inserted_toilet),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert toilet: {}", err))
        }
    }
}

pub async fn update_toilet(
    state: web::Data<AppState>,
    id_toilet: web::Path<String>,
    updated_toilet: web::Json<UpdateToilet>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let toilet_uuid = match Uuid::parse_str(&id_toilet) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format."),
    };

    let target = toilet.filter(id.eq(toilet_uuid));

    match diesel::update(target)
        .set((
            toiletstatus.eq(&updated_toilet.toiletstatus),
            idzone.eq(updated_toilet.idzone),
        ))
        .execute(&mut conn)
    {
        Ok(0) => HttpResponse::NotFound().body("toilet not found."),
        Ok(_) => HttpResponse::Ok().body("toilet updated successfully."),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update toilet: {}", err))
        }
    }
}
