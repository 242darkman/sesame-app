use crate::{
    models::comment::{NewComment, UpdateComment},
    services::comment_service::{create_comment, get_comment, update_comment},
    AppState,
};

use actix_web::{web, Responder};

pub async fn create_comment_controller(
    state: web::Data<AppState>,
    new_comment: web::Json<NewComment>,
) -> impl Responder {
    /// Appeler le service create_location et renvoyer la réponse
    create_comment(state, new_comment).await
}
pub async fn update_comment_controller(
    state: web::Data<AppState>,
    id_comment: web::Path<String>,
    updated_comment: web::Json<UpdateComment>,
) -> impl Responder {
    // Appeler le service update_location et renvoyer la réponse
    update_comment(state, id_comment, updated_comment).await
}
pub async fn get_comment_controller(state: web::Data<AppState>) -> impl Responder {
    get_comment(state).await
}
