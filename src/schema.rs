// @generated automatically by Diesel CLI.

diesel::table! {
    crates (id) {
        id -> Int4,
        rustacean_id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        description -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    rustaceans (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(crates -> rustaceans (rustacean_id));

diesel::allow_tables_to_appear_in_same_query!(
    crates,
    rustaceans,
);
