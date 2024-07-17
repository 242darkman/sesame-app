use crate::{
    models::location::{NewLocation, UpdateLocation},
    services::location_service::{create_location, update_location},
    AppState,
};

use actix_web::{web, Responder};

pub async fn create_location_controller(
    state: web::Data<AppState>,
    new_location: web::Json<NewLocation>,
) -> impl Responder {
    /// Appeler le service create_location et renvoyer la réponse
    create_location(state, new_location).await
}
pub async fn update_location_controller(
    state: web::Data<AppState>,
    id_location: web::Path<String>, // Inclure l'UUID de l'emplacement
    updated_location: web::Json<UpdateLocation>,
) -> impl Responder {
    // Appeler le service update_location et renvoyer la réponse
    update_location(state, id_location, updated_location).await
}
