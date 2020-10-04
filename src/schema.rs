table! {
    comments (id) {
        id -> Int4,
        postId -> Int4,
        body -> Text,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

joinable!(comments -> posts (postId));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
);
