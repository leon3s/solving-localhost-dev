// @generated automatically by Diesel CLI.

diesel::table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
        realm -> Varchar,
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    tokens (id) {
        id -> Uuid,
        value -> Varchar,
        kind -> Varchar,
        user_id -> Uuid,
        created_at -> Timestamp,
        expires_at -> Nullable<Timestamp>,
        session -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_email_verified -> Bool,
    }
}

diesel::joinable!(tokens -> users (user_id));
diesel::joinable!(user_roles -> roles (role_id));
diesel::joinable!(user_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    roles,
    tokens,
    user_roles,
    users,
);
