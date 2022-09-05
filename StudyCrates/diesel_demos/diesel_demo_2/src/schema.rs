// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> BigInt,
        title -> Text,
        body -> Text,
        published -> Bool,
    }
}
