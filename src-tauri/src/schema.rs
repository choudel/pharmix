table! {
    drugs (id) {
        id -> Integer,
        dci -> Text,
        description -> Text,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    drugs,
    posts,
);
