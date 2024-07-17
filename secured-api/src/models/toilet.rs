use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::toilet)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Toilet {
    /// Identifiant unique de la toilet, généré automatiquement.
    pub id: Uuid,
    pub toiletstatus: String,
    pub idzone: Option<Uuid>,
    pub idlevel: Option<Uuid>,
}

pub struct NewToilet {
    pub toiletstatus: String,
    pub idzone: Uuid,
    pub idlevel: Uuid,
}
