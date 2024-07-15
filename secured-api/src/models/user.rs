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
    pub user_id: Uuid,
    /// Prénom de l'utilisateur.
    pub first_name: String,
    /// Nom de famille de l'utilisateur.
    pub last_name: String,
    /// Nom d'utilisateur, qui doit être unique.
    pub username: String,
    /// Adresse email de l'utilisateur, qui doit être unique.
    pub email: String,
    /// Mot de passe de l'utilisateur, stocké sous forme hashée pour la sécurité.
    pub password: String,
    /// Date et heure de création du compte utilisateur.
    pub created_at: NaiveDateTime,
    /// Date et heure de la dernière mise à jour du compte utilisateur.
    pub updated_at: Option<NaiveDateTime>,
}

/// Structure utilisée pour créer un nouvel utilisateur.
///
/// Cette structure simplifie l'insertion de nouveaux utilisateurs dans la base de données
/// en n'incluant pas certains champs qui sont automatiquement générés ou calculés,
/// comme l'`user_id`, `created_at` et `updated_at`.
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    /// Nom d'utilisateur pour le nouvel utilisateur.
    pub username: String,
    /// Prénom de l'utilisateur.
    pub first_name: String,
    /// Nom de famille de l'utilisateur.
    pub last_name: String,
    /// Adresse email pour le nouvel utilisateur.
    pub email: String,
    /// Mot de passe pour le nouvel utilisateur.
    pub password: String,
}

#[derive(AsChangeset, Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
}
