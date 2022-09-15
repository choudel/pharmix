// @generated automatically by Diesel CLI.

diesel::table! {
    drugs (id) {
        id -> Integer,
        dci -> Text,
        description -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    drugs,
    posts,
);
