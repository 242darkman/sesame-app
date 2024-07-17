use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::defaults)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Defaults {
    /// Identifiant unique du defaut, généré automatiquement.
    pub id: Uuid,
    pub defaulttype: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::defaults)]
pub struct NewDefaults {
    pub defaulttype: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::defaults)]
pub struct UpdateDefault {
    pub defaulttype: String,
}
