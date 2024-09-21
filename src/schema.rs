// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        is_public -> Bool,
        #[max_length = 36]
        author -> Varchar,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 36]
        id -> Varchar,
        name -> Varchar,
        email -> Varchar,
        password -> Text,
    }
}

diesel::joinable!(posts -> users (author));

diesel::allow_tables_to_appear_in_same_query!(
    posts,
    users,
);
