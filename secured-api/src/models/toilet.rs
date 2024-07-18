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
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::toilet)]
pub struct NewToilet {
    pub toiletstatus: String,
    pub idzone: Uuid,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::toilet)]
pub struct UpdateToilet {
    pub toiletstatus: String,
    pub idzone: Uuid,
}
