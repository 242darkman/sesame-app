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
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::intervention)]
pub struct NewIntervention {
    pub dateintervention: NaiveDateTime,
    pub interventionstatus: String,
    pub idtoilet: Uuid,
    pub iduser: Uuid,
}
