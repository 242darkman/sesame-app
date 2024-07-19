/// Modèles pour les opérations sur les interventions dans une application utilisant Diesel avec PostgreSQL.
///
/// Ce module définit les structures nécessaires pour manipuler les données des interventions,
/// y compris la création, la mise à jour et la requête des interventions.
use chrono::NaiveDateTime; // Utilisé pour représenter les dates et heures sans fuseau horaire.
use diesel::prelude::*; // Importe les traits et macros essentiels de Diesel.
use serde::{Deserialize, Serialize}; // Traits pour la sérialisation et la désérialisation.
use uuid::Uuid;

use super::comment::Comment;
use super::location::Location;
use super::toilet::Toilet;
use super::zone::Zone; // Utilisé pour les identifiants uniques.

/// Représente une intervention existante dans la base de données.
///
/// Cette structure est utilisée pour interroger les interventions existantes.
#[derive(Queryable, Selectable, Serialize, Insertable, Deserialize, Clone, Debug)]
#[diesel(table_name = crate::schema::intervention)]
#[diesel(check_for_backend(diesel::pg::Pg))] // Assure que cette structure est utilisée avec PostgreSQL.
pub struct Intervention {
    /// Identifiant unique de l'intervention, généré automatiquement.
    pub id: Uuid,
    /// Date et heure de l'intervention.
    pub dateintervention: NaiveDateTime,
    /// Statut de l'intervention.
    pub interventionstatus: String,
    /// Identifiant de la toilette concernée par l'intervention, si applicable.
    pub idtoilet: Option<Uuid>,
    /// Identifiant de l'utilisateur ayant effectué l'intervention, si applicable.
    pub iduser: Option<Uuid>,
    /// Identifiant du défaut associé à l'intervention, si applicable.
    pub iddefault: Option<Uuid>,
    /// Description de l'intervention.
    pub description: String,
}

/// Structure pour créer une nouvelle intervention.
///
/// Utilisée lors de l'insertion d'une nouvelle intervention dans la base de données.
#[derive(Insertable, Deserialize, Serialize, Debug)]
#[diesel(table_name = crate::schema::intervention)]
pub struct NewIntervention {
    pub dateintervention: NaiveDateTime,
    pub interventionstatus: String,
    pub idtoilet: Uuid,
    pub iduser: Uuid,
    pub description: String,
    pub iddefault: Uuid,
}

/// Structure pour mettre à jour une intervention existante.
///
/// Utilisée pour modifier le statut et la description d'une intervention existante.
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::intervention)]
pub struct UpdateIntervention {
    pub interventionstatus: String,
    pub description: String,
}

/// Requête pour créer une intervention.
///
/// Cette structure est utilisée pour recevoir les données de l'utilisateur lors de la création d'une nouvelle intervention.
/// Elle inclut un champ optionnel pour un commentaire associé à l'intervention.
#[derive(Deserialize, Clone, Debug)]
pub struct InterventionRequest {
    pub idlocation: Uuid,
    pub iduser: Uuid,
    pub description: Option<String>,
    pub iddefault: Uuid,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InterventionWithComment {
    pub dateintervention: NaiveDateTime,
    pub interventionstatus: String,
    pub idtoilet: Uuid,
    pub iduser: Uuid,
    pub description: Option<String>,
    pub iddefault: Uuid,
    pub comment: Option<Comment>,
    pub zone: Zone,
    pub location: Location,
    pub toilet: Toilet,
}

#[derive(Serialize, Debug)]
pub struct CreateInterventionWithCommentResponse {
    pub status: String,
    pub interventions: Vec<InterventionWithComment>,
}

#[derive(Serialize, Debug)]
pub struct InterventionWithCommentResponse {
    pub interventions: Vec<InterventionWithComment>,
}
