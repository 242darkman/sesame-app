use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::level)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Level {
    /// Identifiant unique de la zone, généré automatiquement.
    pub id: Uuid,
    pub name: String,
}

pub struct NewLevel {
    pub name: String,
}
