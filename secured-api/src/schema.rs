// @generated automatically by Diesel CLI.

diesel::table! {
    comment (id) {
        id -> Uuid,
        datecomment -> Timestamp,
        comments -> Varchar,
        iduser -> Nullable<Uuid>,
        idintervention -> Nullable<Uuid>,
    }
}

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
    locations (id) {
        id -> Uuid,
        name -> Varchar,
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
        name -> Varchar,
        numlevel -> Int4,
        idlocation -> Nullable<Uuid>,
    }
}

diesel::joinable!(comment -> intervention (idintervention));
diesel::joinable!(comment -> users (iduser));
diesel::joinable!(intervention -> defaults (iddefault));
diesel::joinable!(intervention -> toilet (idtoilet));
diesel::joinable!(intervention -> users (iduser));
diesel::joinable!(toilet -> zone (idzone));
diesel::joinable!(zone -> locations (idlocation));

diesel::allow_tables_to_appear_in_same_query!(
    comment,
    defaults,
    intervention,
    locations,
    toilet,
    users,
    zone,
);
