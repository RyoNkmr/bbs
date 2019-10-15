table! {
    reses (id) {
        id -> Integer,
        thread_slug -> Text,
        user_name -> Text,
        user_id -> Text,
        email -> Text,
        body -> Text,
        ip -> Text,
        created_at -> Timestamp,
    }
}

table! {
    threads (slug) {
        slug -> Text,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(reses -> threads (thread_slug));

allow_tables_to_appear_in_same_query!(
    reses,
    threads,
);
