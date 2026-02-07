// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Int4,
        token -> Varchar,
    }
}

diesel::table! {
    greetings (id) {
        id -> Int4,
        greeting -> Varchar,
        language -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(access_tokens, greetings,);
