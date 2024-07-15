// @generated automatically by Diesel CLI.

diesel::table! {
    _sqlx_migrations (version) {
        version -> Bigint,
        description -> Text,
        installed_on -> Timestamp,
        success -> Bool,
        checksum -> Blob,
        execution_time -> Bigint,
    }
}

diesel::table! {
    products (product_id) {
        product_id -> Unsigned<Integer>,
        #[max_length = 50]
        product_name -> Varchar,
        #[max_length = 30]
        category -> Varchar,
        price -> Decimal,
        stock -> Unsigned<Integer>,
        pharmacy_id -> Unsigned<Integer>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    products,
);
