// @generated automatically by Diesel CLI.

diesel::table! {
    users (user_id) {
        user_id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}
