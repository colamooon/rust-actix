// @generated automatically by Diesel CLI.

diesel::table! {
    member_info (id) {
        id -> Bigint,
        active -> Bool,
        created_at -> Nullable<Datetime>,
        created_id -> Nullable<Bigint>,
        updated_at -> Nullable<Datetime>,
        updated_id -> Nullable<Bigint>,
        username -> Varchar,
        display_name -> Varchar,
        password -> Nullable<Varchar>,
        sign_up_type -> Varchar,
    }
}

diesel::table! {
    post (id) {
        id -> Bigint,
        active -> Bool,
        created_at -> Nullable<Datetime>,
        created_id -> Nullable<Bigint>,
        updated_at -> Nullable<Datetime>,
        updated_id -> Nullable<Bigint>,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    member_info,
    post,
);
