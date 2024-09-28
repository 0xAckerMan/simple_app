// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Varchar,
        description -> Text,
        status -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}
