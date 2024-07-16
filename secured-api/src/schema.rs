// @generated automatically by Diesel CLI.

diesel::table! {
    emplacement (id) {
        id -> Uuid,
        emplacementname -> Varchar,
    }
}

diesel::table! {
    level (id) {
        id -> Uuid,
        idemplacement -> Nullable<Uuid>,
    }
}

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

diesel::joinable!(level -> emplacement (idemplacement));

diesel::allow_tables_to_appear_in_same_query!(
    emplacement,
    level,
    users,
);
