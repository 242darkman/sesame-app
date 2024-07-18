use crate::models::comment::{Comment, NewComment, UpdateComment};
use crate::schema::comment::dsl::*;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use uuid::Uuid;
/**
 * Céatio
 */
pub async fn create_comment(
    state: web::Data<AppState>,
    new_comment: web::Json<NewComment>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let new_comment = NewComment {
        datecomment: new_comment.datecomment.clone(),
        comments: new_comment.comments.clone(),
        iduser: new_comment.iduser.clone(),
        idintervention: new_comment.idintervention.clone(),
    };

    match diesel::insert_into(comment)
        .values(&new_comment)
        .get_result::<Comment>(&mut conn)
    {
        Ok(inserted_comment) => HttpResponse::Created().json(inserted_comment),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert comment: {}", err))
        }
    }
}

pub async fn update_comment(
    state: web::Data<AppState>,
    id_comment: web::Path<String>,
    updated_comment: web::Json<UpdateComment>,
) -> impl Responder {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    let comment_uuid = match Uuid::parse_str(&id_comment) {
        Ok(uuid) => uuid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid UUID format."),
    };

    let target = comment.filter(id.eq(comment_uuid));

    match diesel::update(target)
        .set(comments.eq(&updated_comment.comments))
        .execute(&mut conn)
    {
        Ok(0) => HttpResponse::NotFound().body("toilet not found."),
        Ok(_) => HttpResponse::Ok().body("toilet updated successfully."),
        Err(err) => {
            HttpResponse::InternalServerError().body(format!("Failed to update toilet: {}", err))
        }
    }
}
