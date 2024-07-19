use crate::models::location::Location;
use crate::schema::locations::dsl::*;
use crate::AppState;
use actix_web::web;
use diesel::prelude::*;

/// Récupère toutes les localisations disponibles dans la base de données.
///
/// # Arguments
///
/// * `state` - Un état d'application contenant une connexion à la base de données.
///
/// # Retourne
///
/// Cette fonction retourne un `Result<Vec<Location>, actix_web::Error>`. En cas de succès, elle renvoie un vecteur contenant toutes les localisations.
/// En cas d'échec, elle renvoie une erreur `actix_web::Error` avec un message indiquant la raison de l'échec.
///
/// # Exemples
///
/// ```
/// let app_state = web::Data::new(AppState {
///     conn: /* initialisez ici votre pool de connexions à la base de données */,
/// });
///
/// let locations_result = get_locations(app_state).await;
/// assert!(locations_result.is_ok());
/// ```
///
/// # Paniques
///
/// Cette fonction panique si elle ne parvient pas à obtenir une connexion à partir du pool.
pub async fn get_locations(state: web::Data<AppState>) -> Result<Vec<Location>, actix_web::Error> {
    let mut conn = state
        .conn
        .get()
        .expect("Failed to get a connection from the pool.");

    match locations.load::<Location>(&mut conn) {
        Ok(all_locations) => Ok(all_locations),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Failed to retrieve locations: {}",
            err
        ))),
    }
}
