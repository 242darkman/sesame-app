use crate::{
    models::toilet::{NewToilet, UpdateToilet},
    services::toilet_service::{create_toilet, update_toilet},
    AppState,
};

use actix_web::{web, Responder};

pub async fn create_toilet_controller(
    state: web::Data<AppState>,
    new_toilet: web::Json<NewToilet>,
) -> impl Responder {
    /// Appeler le service create_location et renvoyer la réponse
    create_toilet(state, new_toilet).await
}
pub async fn update_toilet_controller(
    state: web::Data<AppState>,
    id_toilet: web::Path<String>, // Inclure l'UUID de l'emplacement
    updated_toilet: web::Json<UpdateToilet>,
) -> impl Responder {
    // Appeler le service update_location et renvoyer la réponse
    update_toilet(state, id_toilet, updated_toilet).await
}
