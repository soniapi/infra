// @generated automatically by Diesel CLI.

diesel::table! {
    objects (id) {
        id -> Int4,
        d -> Timestamp,
        t -> Text,
        p -> Float4,
        s -> Float4,
    }
}
