// @generated automatically by Diesel CLI.

diesel::table! {
    greetings (id) {
        id -> Int4,
        greeting -> Varchar,
        language -> Varchar,
    }
}
