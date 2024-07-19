use crate::models::comment::{Comment, NewComment};
use crate::models::intervention::{
    CreateInterventionWithCommentResponse, Intervention, InterventionRequest,
    InterventionWithComment, InterventionWithCommentResponse, NewIntervention,
};

use crate::models::location::Location;
use crate::models::toilet::Toilet;
use crate::models::zone::Zone;
use crate::schema::comment::dsl::*;
use crate::schema::intervention::dsl::*;
use crate::schema::{locations, toilet, users, zone};
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;

pub async fn create_intervention(
    state: web::Data<AppState>,
    new_intervention_req: web::Json<InterventionRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    println!("Creating new intervention: {:?}", new_intervention_req);

    // Retrieve user ID from keycloak_uuid
    let user_id = users::table
        .filter(users::keycloak_uuid.eq(new_intervention_req.iduser))
        .select(users::id)
        .first::<Uuid>(&mut conn)
        .map_err(|e| {
            println!("Error finding user by keycloak_uuid: {}", e);
            actix_web::error::ErrorBadRequest("Error finding user by keycloak_uuid")
        })?;

    // Find the zone by location ID
    let zone = zone::table
        .filter(crate::schema::zone::idlocation.eq(new_intervention_req.idlocation))
        .first::<Zone>(&mut conn)
        .map_err(|e| {
            println!("Error finding zone by location ID: {}", e);
            actix_web::error::ErrorBadRequest("Error finding zone by location ID")
        })?;

    // Find the toilet by zone ID
    let toilet = toilet::table
        .filter(crate::schema::toilet::idzone.eq(zone.id))
        .first::<Toilet>(&mut conn)
        .map_err(|e| {
            println!("Error finding toilet by zone ID: {}", e);
            actix_web::error::ErrorBadRequest("Error finding toilet by zone ID")
        })?;

    // Find the location by zone ID
    let location_id = zone.idlocation.ok_or_else(|| {
        println!("Location ID is missing in zone");
        actix_web::error::ErrorBadRequest("Location ID is missing in zone")
    })?;
    let location = locations::table
        .filter(crate::schema::locations::id.eq(location_id))
        .first::<Location>(&mut conn)
        .map_err(|e| {
            println!("Error finding location by zone ID: {}", e);
            actix_web::error::ErrorBadRequest("Error finding location by zone ID")
        })?;

    // Create new intervention
    let new_intervention = NewIntervention {
        dateintervention: chrono::Utc::now().naive_utc(),
        interventionstatus: "created".to_string(),
        idtoilet: toilet.id,
        iduser: user_id,
        description: new_intervention_req.description.clone().unwrap_or_default(),
        iddefault: new_intervention_req.iddefault,
    };

    println!(
        "New intervention from intervention_service: {:?}",
        new_intervention
    );

    let intervention_id_value = diesel::insert_into(intervention)
        .values(&new_intervention)
        .returning(crate::schema::intervention::id)
        .get_result::<Uuid>(&mut conn)
        .map_err(|e| {
            println!("Error saving new intervention: {}", e);
            actix_web::error::ErrorInternalServerError("Error saving new intervention")
        })?;

    // Create new comment if provided
    let mut comment_data = None;
    if let Some(comment_text) = &new_intervention_req.comment {
        let new_comment = NewComment {
            datecomment: chrono::Utc::now().naive_utc(),
            comments: comment_text.clone(),
            iduser: Some(user_id),
            idintervention: Some(intervention_id_value),
        };

        let comment_id_value = diesel::insert_into(comment)
            .values(&new_comment)
            .returning(crate::schema::comment::id)
            .get_result::<Uuid>(&mut conn)
            .map_err(|e| {
                println!("Error saving new comment: {}", e);
                actix_web::error::ErrorInternalServerError("Error saving new comment")
            })?;

        comment_data = Some(Comment {
            id: comment_id_value,
            datecomment: new_comment.datecomment,
            comments: new_comment.comments,
            iduser: new_comment.iduser,
            idintervention: new_comment.idintervention,
        });
    }

    // Prepare response
    let response = CreateInterventionWithCommentResponse {
        status: "Ok".to_string(),
        interventions: vec![InterventionWithComment {
            dateintervention: new_intervention.dateintervention,
            interventionstatus: new_intervention.interventionstatus,
            idtoilet: new_intervention.idtoilet,
            iduser: new_intervention.iduser,
            description: Some(new_intervention.description),
            iddefault: new_intervention.iddefault,
            comment: comment_data,
            zone,
            location,
            toilet,
        }],
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_interventions_with_comments(
    state: web::Data<AppState>,
) -> Result<impl Responder, actix_web::Error> {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let all_interventions = intervention
        .load::<Intervention>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_comments = comment
        .load::<Comment>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_toilets = toilet::table
        .load::<Toilet>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_zones = zone::table
        .load::<Zone>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_locations = locations::table
        .load::<Location>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut interventions_with_comments: Vec<InterventionWithComment> = Vec::new();

    for single_intervention in all_interventions {
        let associated_comment = all_comments
            .iter()
            .find(|c| c.idintervention == Some(single_intervention.id))
            .cloned();

        let associated_toilet = all_toilets
            .iter()
            .find(|t| t.id == single_intervention.idtoilet.unwrap_or(uuid::Uuid::nil()))
            .cloned()
            .ok_or_else(|| {
                actix_web::error::ErrorInternalServerError(
                    "Toilet not found for the given intervention",
                )
            })?;

        let associated_zone = all_zones
            .iter()
            .find(|z| z.id == associated_toilet.idzone.unwrap())
            .cloned()
            .ok_or_else(|| {
                actix_web::error::ErrorInternalServerError("Zone not found for the given toilet")
            })?;

        let associated_location = all_locations
            .iter()
            .find(|l| l.id == associated_zone.idlocation.unwrap())
            .cloned()
            .ok_or_else(|| {
                actix_web::error::ErrorInternalServerError("Location not found for the given zone")
            })?;

        let intervention_with_comment = InterventionWithComment {
            dateintervention: single_intervention.dateintervention,
            interventionstatus: single_intervention.interventionstatus.clone(),
            idtoilet: single_intervention.idtoilet.unwrap_or(uuid::Uuid::nil()),
            iduser: single_intervention.iduser.unwrap_or(uuid::Uuid::nil()),
            description: Some(single_intervention.description.clone()),
            iddefault: single_intervention.iddefault.unwrap_or(uuid::Uuid::nil()),
            comment: associated_comment,
            zone: associated_zone,
            location: associated_location,
            toilet: associated_toilet,
        };

        interventions_with_comments.push(intervention_with_comment);
    }

    let response = InterventionWithCommentResponse {
        interventions: interventions_with_comments,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_intervention_with_comments(
    state: web::Data<AppState>,
    interv_id: web::Path<Uuid>,
) -> Result<HttpResponse, actix_web::Error> {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let single_intervention = intervention
        .filter(crate::schema::intervention::id.eq(*interv_id))
        .first::<Intervention>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let all_comments = comment
        .load::<Comment>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let associated_comment = all_comments
        .iter()
        .find(|c| c.idintervention == Some(single_intervention.id))
        .cloned();

    let associated_toilet = toilet::table
        .filter(toilet::id.eq(single_intervention.idtoilet.unwrap_or(uuid::Uuid::nil())))
        .first::<Toilet>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let associated_zone = zone::table
        .filter(zone::id.eq(associated_toilet.idzone.unwrap()))
        .first::<Zone>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let associated_location = locations::table
        .filter(locations::id.eq(associated_zone.idlocation.unwrap()))
        .first::<Location>(&mut conn)
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let intervention_with_comment = InterventionWithComment {
        dateintervention: single_intervention.dateintervention,
        interventionstatus: single_intervention.interventionstatus.clone(),
        idtoilet: single_intervention.idtoilet.unwrap_or(uuid::Uuid::nil()),
        iduser: single_intervention.iduser.unwrap_or(uuid::Uuid::nil()),
        description: Some(single_intervention.description.clone()),
        iddefault: single_intervention.iddefault.unwrap_or(uuid::Uuid::nil()),
        comment: associated_comment,
        zone: associated_zone,
        location: associated_location,
        toilet: associated_toilet,
    };

    let response = InterventionWithCommentResponse {
        interventions: vec![intervention_with_comment],
    };

    Ok(HttpResponse::Ok().json(response))
}
