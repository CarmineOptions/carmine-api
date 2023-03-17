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
