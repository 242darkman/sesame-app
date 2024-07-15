use crate::models::user::UpdateUser;
use crate::utils::pagination_param::PaginationParams;
use crate::{
    models::user::NewUser, repository::user_repository::UserRepository, utils::app_state::AppState,
    utils::error_response::ErrorResponse,
};
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

/// Crée un nouvel utilisateur et retourne une réponse HTTP indiquant le résultat de l'opération.
///
/// Cette fonction effectue les vérifications nécessaires pour s'assurer que le nom d'utilisateur et l'email fournis ne sont pas déjà utilisés. Si l'un des deux est déjà enregistré, une réponse HTTP de conflit est retournée. Si les vérifications sont passées, l'utilisateur est créé dans la base de données.
///
/// # Arguments
///
/// * `new_user` - Un `web::Json<NewUser>` contenant les données du nouvel utilisateur à créer.
/// * `pool` - Un `web::Data<AppState>` représentant le pool de connexions à la base de données.
///
/// # Retourne
///
/// Une réponse HTTP qui peut être :
/// - `Created` avec les données de l'utilisateur créé si l'opération réussit.
/// - `Conflict` avec un message d'erreur si le nom d'utilisateur ou l'email est déjà utilisé.
/// - `InternalServerError` si une erreur se produit lors de la création de l'utilisateur.
pub async fn create_user(
    new_user: web::Json<NewUser>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let mut conn = pool
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    // Vérifie si le nom d'utilisateur existe déjà.
    if let Ok(Some(_)) = UserRepository::get_by_username(new_user.username.clone(), &mut conn).await
    {
        let response = ErrorResponse {
            message: "Username already exists".to_string(),
            error_code: 409,
        };
        return HttpResponse::Conflict().json(response);
    }

    // Vérifie si l'email existe déjà.
    if let Ok(Some(_)) = UserRepository::get_by_email(new_user.email.clone(), &mut conn).await {
        let response = ErrorResponse {
            message: "Email already exists".to_string(),
            error_code: 409,
        };
        return HttpResponse::Conflict().json(response);
    }

    // Crée l'utilisateur.
    match UserRepository::create(new_user.into_inner(), &mut conn).await {
        Ok(user) => {
            let response = serde_json::json!({
                "users": [user],
                "meta": {}
            });
            HttpResponse::Created().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Récupère tous les utilisateurs avec pagination et retourne la liste paginée des utilisateurs ainsi que des métadonnées associées.
///
/// # Arguments
///
/// * `pool` - Un `web::Data<AppState>` représentant le pool de connexions à la base de données.
/// * `pagination` - Les paramètres de pagination encapsulés dans `web::Query<PaginationParams>` incluant la limite et l'offset.
///
/// # Retourne
///
/// Une réponse HTTP qui peut être :
/// - `Ok` avec la liste paginée des utilisateurs et des métadonnées associées si l'opération réussit.
/// - `InternalServerError` si une erreur se produit lors de la récupération des utilisateurs.
///
/// Les métadonnées incluent le total des utilisateurs, le nombre total de pages, le nombre d'utilisateurs par page, et
/// les indicateurs pour les pages suivante et précédente si applicables.
pub async fn get_users(
    pool: web::Data<AppState>,
    web::Query(pagination): web::Query<PaginationParams>,
) -> impl Responder {
    let mut conn = pool
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    let limit = pagination.limit.unwrap_or(10);
    let offset = pagination.offset.unwrap_or(0);

    match UserRepository::get_all(&mut conn, limit, offset).await {
        Ok((users, total_count)) => {
            let total_pages = (total_count as f64 / limit as f64).ceil() as i64;
            let meta = serde_json::json!({
                "total": total_count,
                "total_pages": total_pages,
                "per_page": limit,
                "next": if offset + limit < total_count { Some(offset + limit) } else { None },
                "previous": if offset > 0 { Some(offset - limit) } else { None },
            });
            let response = serde_json::json!({
                "users": users,
                "meta": meta,
            });
            HttpResponse::Ok().json(response)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Récupère un utilisateur par son ID et retourne cet utilisateur.
///
/// # Arguments
///
/// * `user_id` - Un `web::Path<String>` contenant l'ID de l'utilisateur à récupérer.
/// * `pool` - Un `web::Data<AppState>` représentant le pool de connexions à la base de données.
///
/// # Retourne
///
/// Une réponse HTTP qui peut être :
/// - `Ok` avec l'utilisateur trouvé encapsulé dans un objet JSON si l'opération réussit.
/// - `InternalServerError` si une erreur se produit lors de la récupération de l'utilisateur.
/// - `BadRequest` si l'ID fourni n'est pas un UUID valide.
pub async fn get_user_by_id(
    user_id: web::Path<String>,
    pool: web::Data<AppState>,
) -> impl Responder {
    let mut conn = pool
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match Uuid::parse_str(&user_id) {
        Ok(uuid_id) => match UserRepository::get_by_id(uuid_id, &mut conn).await {
            Ok(user) => {
                let response = serde_json::json!({
                    "users": [user],
                    "meta": {}
                });
                HttpResponse::Ok().json(response)
            }
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

/// Met à jour un utilisateur et retourne l'utilisateur mis à jour.
///
/// # Arguments
///
/// * `user_id` - Un chemin web::Path<String> contenant l'ID de l'utilisateur à mettre à jour.
/// * `new_user` - Les nouvelles données de l'utilisateur encapsulées dans web::Json<UpdateUser>.
/// * `pool` - Le pool de connexions à la base de données encapsulé dans web::Data<AppState>.
///
/// # Retourne
///
/// Une réponse HTTP qui peut être :
/// - `Ok` avec l'utilisateur mis à jour si l'opération réussit.
/// - `InternalServerError` si une erreur se produit lors de la mise à jour ou de la récupération de l'utilisateur.
/// - `BadRequest` si l'ID fourni n'est pas un UUID valide.
pub async fn update_user(
    user_id: web::Path<String>,
    new_user: web::Json<UpdateUser>,
    pool: web::Data<AppState>,
) -> impl Responder {
    // Obtention d'une connexion à la base de données à partir du pool.
    let mut conn = pool
        .conn
        .get()
        .expect("Impossible d'obtenir une connexion à la base de données depuis le pool");

    // Tentative de conversion de l'ID utilisateur de String à Uuid.
    match Uuid::parse_str(&user_id) {
        Ok(uuid_id) => {
            match UserRepository::update(uuid_id, new_user.into_inner(), &mut conn).await {
                Ok(_) => match UserRepository::get_by_id(uuid_id, &mut conn).await {
                    Ok(user) => {
                        let response = serde_json::json!({
                            "users": [user],
                            "meta": {}
                        });
                        HttpResponse::Ok().json(response)
                    }
                    Err(_) => HttpResponse::InternalServerError().finish(),
                },
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

/// Supprime un utilisateur spécifié par son ID.
///
/// # Arguments
///
/// * `user_id` - Un chemin contenant l'ID de l'utilisateur à supprimer, sous forme de chaîne.
/// * `pool` - Le pool de connexions à la base de données encapsulé dans `web::Data<AppState>`.
///
/// # Retourne
///
/// Une réponse HTTP qui peut être :
/// - `Ok` avec un corps JSON vide si la suppression est réussie.
/// - `InternalServerError` si une erreur se produit lors de la suppression.
/// - `BadRequest` si l'ID fourni n'est pas valide.
pub async fn delete_user(user_id: web::Path<String>, pool: web::Data<AppState>) -> impl Responder {
    let mut conn = pool
        .conn
        .get()
        .expect("couldn't get db connection from pool");

    match Uuid::parse_str(&user_id) {
        Ok(uuid_id) => match UserRepository::delete(uuid_id, &mut conn).await {
            Ok(_) => {
                let response = serde_json::json!({
                    "users": [],
                    "meta": {}
                });
                HttpResponse::Ok().json(response)
            }
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
