use actix_web::web;

use crate::controllers::user_controller::{
    create_user, delete_user, get_user_by_id, get_users, update_user,
};

pub fn init_user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::get().to(get_users))
            .route("/{user_id}", web::get().to(get_user_by_id))
            .route("", web::post().to(create_user))
            .route("/{user_id}", web::put().to(update_user))
            .route("/{user_id}", web::delete().to(delete_user)),
    );
}
