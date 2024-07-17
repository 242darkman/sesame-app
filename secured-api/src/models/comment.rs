use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize, Insertable, Deserialize)]
#[diesel(table_name = crate::schema::comment)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    /// Identifiant unique du commentaire, généré automatiquement.
    pub id: Uuid,
    pub datecomment: NaiveDateTime,
    pub comments: String,
    pub iduser: Option<Uuid>,
    pub idintervention: Option<Uuid>,
}
#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::comment)]
pub struct NewComment {
    pub datecomment: NaiveDateTime,
    pub comments: String,
    pub iduser: Option<Uuid>,
    pub idintervention: Option<Uuid>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = crate::schema::comment)]
pub struct UpdateComment {
    pub comments: String,
}
