// @generated automatically by Diesel CLI.

diesel::table! {
    defaults (id) {
        id -> Uuid,
        defaulttype -> Varchar,
    }
}

diesel::table! {
    intervention (id) {
        id -> Uuid,
        dateintervention -> Timestamp,
        interventionstatus -> Varchar,
        idtoilet -> Nullable<Uuid>,
        iduser -> Nullable<Uuid>,
        iddefault -> Nullable<Uuid>,
        description -> Varchar,
    }
}

diesel::table! {
    level (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    toilet (id) {
        id -> Uuid,
        toiletstatus -> Varchar,
        idzone -> Nullable<Uuid>,
        idlevel -> Nullable<Uuid>,
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
        name -> Varchar,
    }
}

diesel::joinable!(intervention -> defaults (iddefault));
diesel::joinable!(intervention -> toilet (idtoilet));
diesel::joinable!(intervention -> users (iduser));
diesel::joinable!(toilet -> level (idlevel));
diesel::joinable!(toilet -> zone (idzone));

diesel::allow_tables_to_appear_in_same_query!(
    defaults,
    intervention,
    level,
    toilet,
    users,
    zone,
);
