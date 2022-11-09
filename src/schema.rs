// @generated automatically by Diesel CLI.

diesel::table! {
    member_info (id) {
        id -> Int4,
        active -> Bool,
        created_at -> Nullable<Timestamp>,
        created_id -> Nullable<Int8>,
        updated_at -> Nullable<Timestamp>,
        updated_id -> Nullable<Int8>,
        username -> Varchar,
        display_name -> Varchar,
        password -> Nullable<Varchar>,
        sign_up_type -> Varchar,
    }
}

diesel::table! {
    post (id) {
        id -> Int4,
        active -> Bool,
        created_at -> Nullable<Timestamp>,
        created_id -> Nullable<Int8>,
        updated_at -> Nullable<Timestamp>,
        updated_id -> Nullable<Int8>,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    member_info,
    post,
);
