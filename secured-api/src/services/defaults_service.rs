use crate::models::defaults::{Defaults, NewDefaults, UpdateDefault};
use crate::schema::defaults::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;

/// Crée un nouvel defaults
///
/// # Arguments
///
/// * `state` - L'état de l'application contenant le pool de connexions
/// * `new_defaults` - Les données du nouvel defaut à créer
///
/// # Retourne
///
/// * `HttpResponse` - La réponse HTTP contenant le defaut créé ou une erreur
pub async fn create_defaults(
    state: web::Data<AppState>,
    new_defaults: web::Json<NewDefaults>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_defaults = NewDefaults {
        defaulttype: new_defaults.defaulttype.clone(),
    };

    match diesel::insert_into(defaults)
        .values(&new_defaults)
        .get_result::<Defaults>(&mut conn)
    {
        Ok(inserted_defaults) => HttpResponse::Created().json(inserted_defaults),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert default: {}", err))
        }
    }
}

/// Met à jour un defaults existant
///
/// # Arguments
///
/// * `state` - L'état de l'application contenant le pool de connexions
/// * `id_defaults` - L'identifiant du defauts à mettre à jour
/// * `updated_defaults` - Les nouvelles données du defaults
///
/// # Retourne
///
/// * `HttpResponse` - La réponse HTTP indiquant le succès ou l'échec de la mise à jour
pub async fn update_defaults(
    state: web::Data<AppState>,
    id_defaults: web::Path<String>,
    updated_defaults: web::Json<UpdateDefault>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let defaults_uuid = match Uuid::parse_str(&id_defaults) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format."),
    };

    let target = defaults.filter(id.eq(defaults_uuid));

    match diesel::update(target)
        .set(defaulttype.eq(&updated_defaults.defaulttype))
        .execute(&mut conn)
    {
        Ok(0) => HttpResponse::NotFound().body("Default not found."),
        Ok(_) => HttpResponse::Ok().body("Defaults updated successfully."),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update default: {}", err))
        }
    }
}
