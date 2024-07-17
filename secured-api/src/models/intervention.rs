use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::intervention)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Intervention {
    /// Identifiant unique de la toilet, généré automatiquement.
    pub id: Uuid,
    pub dateintervention: NaiveDateTime,
    pub interventionstatus: String,
    pub idtoilet: Option<Uuid>,
    pub iduser: Option<Uuid>,
    pub description: String,
    pub iddefault: Option<Uuid>,
}

pub struct NewIntervention {
    pub dateintervention: NaiveDateTime,
    pub interventionstatus: String,
    pub idtoilet: Uuid,
    pub iduser: Uuid,
    pub description: String,
    pub iddefault: Uuid,
}
