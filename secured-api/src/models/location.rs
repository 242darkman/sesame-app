use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::locations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Location {
    /// Identifiant unique de la emplacement, généré automatiquement.
    pub id: Uuid,
    pub name: String,
}
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::locations)]
pub struct NewLocation {
    pub name: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::locations)]
pub struct UpdateLocation {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LocationsResponse {
    pub locations: Vec<Location>,
}
