// @generated automatically by Diesel CLI.

diesel::table! {
    responses (id) {
        id -> Integer,
        user_name -> Text,
        user_id -> Text,
        email -> Text,
        body -> Text,
        ip -> Text,
        created_at -> Timestamp,
    }
}
