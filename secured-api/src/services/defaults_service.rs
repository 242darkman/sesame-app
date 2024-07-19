use crate::models::defaults::Defaults;
use crate::schema::defaults::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

/// Récupère tous les types de problèmes (defaults) disponibles dans la base de données.
///
/// # Arguments
///
/// * `state` - Un état d'application contenant une connexion à la base de données.
///
/// # Retourne
///
/// Cette fonction retourne un `Result<Vec<Defaults>, actix_web::Error>`. En cas de succès, elle renvoie un vecteur contenant tous les types de problèmes.
/// En cas d'échec, elle renvoie une erreur `actix_web::Error` avec un message indiquant la raison de l'échec.
///
/// # Exemples
///
/// ```
/// let app_state = web::Data::new(AppState {
///     conn: /* initialisez ici votre pool de connexions à la base de données */,
/// });
///
/// let problem_types_result = get_problem_types(app_state).await;
/// assert!(problem_types_result.is_ok());
/// ```
///
/// # Paniques
///
/// Cette fonction panique si elle ne parvient pas à obtenir une connexion à partir du pool.
pub async fn get_problem_types(
    state: web::Data<AppState>,
) -> Result<Vec<Defaults>, actix_web::Error> {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match defaults.load::<Defaults>(&mut conn) {
        Ok(all_default_types) => Ok(all_default_types),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Failed to retrieve zones: {}",
            err
        ))),
    }
}
