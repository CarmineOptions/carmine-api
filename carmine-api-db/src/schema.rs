// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        block_hash -> Text,
        block_number -> Int8,
        transaction_hash -> Text,
        event_index -> Int8,
        from_address -> Text,
        timestamp -> Int8,
        action -> Text,
        caller -> Text,
        option_token -> Text,
        capital_transfered -> Text,
        option_tokens_minted -> Text,
    }
}

diesel::table! {
    options (id) {
        id -> Int4,
        option_side -> Int2,
        maturity -> Int8,
        strike_price -> Text,
        quote_token_address -> Text,
        base_token_address -> Text,
        option_type -> Int2,
        option_address -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    events,
    options,
);
