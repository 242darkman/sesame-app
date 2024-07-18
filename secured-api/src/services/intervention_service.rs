use crate::models::intervention::{Intervention, NewIntervention, UpdateIntervention};
use crate::schema::intervention::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;

/// Crée une nouvelle intervention
///
/// # Arguments
///
/// * `state` - L'état de l'application contenant le pool de connexions
/// * `new_intervention` - Les données de la nouvelle inytervention à créer
///
/// # Retourne
///
/// * `HttpResponse` - La réponse HTTP contenant l'intervention créé ou une erreur
pub async fn create_intervention(
    state: web::Data<AppState>,
    new_intervention: web::Json<NewIntervention>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_intervention = NewIntervention {
        dateintervention: new_intervention.dateintervention.clone(),
        interventionstatus: new_intervention.interventionstatus.clone(),
        idtoilet: new_intervention.idtoilet.clone(),
        iduser: new_intervention.iduser.clone(),
        description: new_intervention.description.clone(),
        iddefault: new_intervention.iddefault.clone(),
    };

    match diesel::insert_into(intervention)
        .values(&new_intervention)
        .get_result::<Intervention>(&mut conn)
    {
        Ok(inserted_inetrvention) => HttpResponse::Created().json(inserted_inetrvention),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to insert intervention: {}", err)),
    }
}

/// Met à jour une intervention existant
///
/// # Arguments
///
/// * `state` - L'état de l'application contenant le pool de connexions
/// * `id_intervention` - L'identifiant de l'intervention à mettre à jour
/// * `updated_intervention` - Les nouvelles données de l'intervention
///
/// # Retourne
///
/// * `HttpResponse` - La réponse HTTP indiquant le succès ou l'échec de la mise à jour
pub async fn update_intervention(
    state: web::Data<AppState>,
    id_intervention: web::Path<String>,
    updated_intervention: web::Json<UpdateIntervention>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let intervention_uuid = match Uuid::parse_str(&id_intervention) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format."),
    };

    let target = intervention.filter(id.eq(intervention_uuid));

    match diesel::update(target)
        .set((
            interventionstatus.eq(&updated_intervention.interventionstatus),
            description.eq(&updated_intervention.description),
        ))
        .execute(&mut conn)
    {
        Ok(0) => HttpResponse::NotFound().body("intervention not found."),
        Ok(_) => HttpResponse::Ok().body("intervention updated successfully."),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update toilet: {}", err))
        }
    }
}
/// Récupère toutes les interventions
///
/// # Arguments
///
/// * `state` - L'état de l'application contenant le pool de connexions
///
/// # Retourne
///
/// * `HttpResponse` - La réponse HTTP contenant la liste des interventions ou une erreur
pub async fn get_interventions(state: web::Data<AppState>) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");
    match intervention.load::<Intervention>(&mut conn) {
        Ok(all_interventions) => HttpResponse::Ok().json(all_interventions),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to retrieve interventions: {}", err)),
    }
}
