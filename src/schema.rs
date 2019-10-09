table! {
    reses (id) {
        id -> Integer,
        thread_id -> Integer,
        user_name -> Text,
        user_id -> Text,
        email -> Text,
        body -> Text,
        ip -> Text,
        created_at -> Timestamp,
    }
}

table! {
    threads (id) {
        id -> Integer,
        slug -> Text,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(reses -> threads (thread_id));

allow_tables_to_appear_in_same_query!(
    reses,
    threads,
);
