// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        firstname -> Varchar,
        lastname -> Varchar,
        email -> Varchar,
        keycloak_uuid -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
