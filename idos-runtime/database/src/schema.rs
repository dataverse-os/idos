// @generated automatically by Diesel CLI.

diesel::table! {
    statistics (date) {
        date -> Timestamptz,
        data -> Jsonb,
    }
}
