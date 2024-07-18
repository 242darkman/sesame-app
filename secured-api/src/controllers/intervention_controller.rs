use crate::{
    models::intervention::{NewIntervention, UpdateIntervention},
    services::intervention_service::{create_intervention, update_intervention},
    AppState,
};

use actix_web::{web, Responder};

pub async fn create_intervention_controller(
    state: web::Data<AppState>,
    new_intervention: web::Json<NewIntervention>,
) -> impl Responder {
    /// Appeler le service create_location et renvoyer la réponse
    create_intervention(state, new_intervention).await
}
pub async fn update_intervention_controller(
    state: web::Data<AppState>,
    id_intervention: web::Path<String>, // Inclure l'UUID de l'emplacement
    updated_intervention: web::Json<UpdateIntervention>,
) -> impl Responder {
    // Appeler le service update_location et renvoyer la réponse
    update_intervention(state, id_intervention, updated_intervention).await
}
