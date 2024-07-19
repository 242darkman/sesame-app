use crate::models::zone::Zone;
use crate::schema::zone::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

/// Récupère toutes les zones disponibles dans la base de données.
///
/// # Arguments
///
/// * `state` - Un état d'application contenant une connexion à la base de données.
///
/// # Retourne
///
/// Cette fonction retourne un `Result<Vec<Zone>, actix_web::Error>`. En cas de succès, elle renvoie un vecteur contenant toutes les zones.
/// En cas d'échec, elle renvoie une erreur `actix_web::Error` avec un message indiquant la raison de l'échec.
///
/// # Exemples
///
/// ```
/// let app_state = web::Data::new(AppState {
///     conn: /* initialisez ici votre pool de connexions à la base de données */,
/// });
///
/// let zones_result = get_zones(app_state).await;
/// assert!(zones_result.is_ok());
/// ```
///
/// # Paniques
///
/// Cette fonction panique si elle ne parvient pas à obtenir une connexion à partir du pool.
pub async fn get_zones(state: web::Data<AppState>) -> Result<Vec<Zone>, actix_web::Error> {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match zone.load::<Zone>(&mut conn) {
        Ok(all_zones) => Ok(all_zones),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Failed to retrieve zones: {}",
            err
        ))),
    }
}
