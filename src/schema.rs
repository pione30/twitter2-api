table! {
    posts (id) {
        id -> Uuid,
        body -> Text,
        user_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
