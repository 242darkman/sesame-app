use crate::{
    models::zone::{NewZone, UpdateZone},
    services::zone_service::{create_zone, update_zone},
    AppState,
};

use actix_web::{web, Responder};

pub async fn create_zone_controller(
    state: web::Data<AppState>,
    new_zone: web::Json<NewZone>,
) -> impl Responder {
    /// Appeler le service create_location et renvoyer la réponse
    create_zone(state, new_zone).await
}
pub async fn update_zone_controller(
    state: web::Data<AppState>,
    id_zone: web::Path<String>, // Inclure l'UUID de l'emplacement
    updated_zone: web::Json<UpdateZone>,
) -> impl Responder {
    // Appeler le service update_location et renvoyer la réponse
    update_zone(state, id_zone, updated_zone).await
}
