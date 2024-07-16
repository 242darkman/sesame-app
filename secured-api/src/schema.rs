// @generated automatically by Diesel CLI.

diesel::table! {
    emplacement (id) {
        id -> Uuid,
        emplacementname -> Varchar,
    }
}

diesel::table! {
    intervention (id) {
        id -> Uuid,
        dateintervention -> Timestamp,
        interventionstatus -> Varchar,
        idtoilet -> Nullable<Uuid>,
        iduser -> Nullable<Uuid>,
    }
}

diesel::table! {
    level (id) {
        id -> Uuid,
        idemplacement -> Nullable<Uuid>,
    }
}

diesel::table! {
    toilet (id) {
        id -> Uuid,
        toiletstatus -> Varchar,
        idzone -> Nullable<Uuid>,
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

diesel::table! {
    zone (id) {
        id -> Uuid,
        zonename -> Varchar,
        idlevel -> Nullable<Uuid>,
    }
}

diesel::joinable!(intervention -> toilet (idtoilet));
diesel::joinable!(intervention -> users (iduser));
diesel::joinable!(level -> emplacement (idemplacement));
diesel::joinable!(toilet -> zone (idzone));
diesel::joinable!(zone -> level (idlevel));

diesel::allow_tables_to_appear_in_same_query!(
    emplacement,
    intervention,
    level,
    toilet,
    users,
    zone,
);
