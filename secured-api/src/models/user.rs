use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Représente un utilisateur dans la base de données.
///
/// Cette structure est utilisée pour interagir avec la table `users` dans la base de données.
/// Elle inclut des champs pour toutes les colonnes de la table et utilise des traits de Diesel
/// pour faciliter les opérations de base de données comme les requêtes et les insertions.
#[derive(Queryable, Selectable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    /// Identifiant unique de l'utilisateur, généré automatiquement.
    pub id: Uuid,
    /// Prénom de l'utilisateur.
    pub firstname: String,
    /// Nom de famille de l'utilisateur.
    pub lastname: String,
    /// Adresse email de l'utilisateur, qui doit être unique.
    pub email: String,
    /// UUID de Keycloak pour l'utilisateur, utilisé pour les autorisations.
    pub keycloak_uuid: Uuid,
    /// Date et heure de création du compte utilisateur.
    pub created_at: NaiveDateTime,
    /// Date et heure de la dernière mise à jour du compte utilisateur.
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
pub struct UserChangeset {
    pub id: Option<Uuid>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub keycloak_uuid: Option<Uuid>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub keycloak_uuid: Uuid,
}
