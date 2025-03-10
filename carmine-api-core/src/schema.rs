// @generated automatically by Diesel CLI.

diesel::table! {
    events (transaction_hash) {
        block_hash -> Text,
        block_number -> Int8,
        transaction_hash -> Text,
        event_index -> Int8,
        from_address -> Text,
        timestamp -> Int8,
        action -> Text,
        caller -> Text,
        token_address -> Text,
        capital_transfered -> Text,
        tokens_minted -> Text,
    }
}

diesel::table! {
    options (option_address) {
        option_side -> Int2,
        maturity -> Int8,
        strike_price -> Text,
        quote_token_address -> Text,
        base_token_address -> Text,
        option_type -> Int2,
        option_address -> Text,
        lp_address -> Text,
    }
}

diesel::table! {
    blocks (block_number) {
        block_number -> Int8,
        timestamp -> Int8,
    }
}

diesel::table! {
    pools (lp_address) {
        lp_address -> Text,
    }
}

diesel::table! {
    pool_state (lp_address, block_number) {
        unlocked_cap -> Text,
        locked_cap -> Text,
        lp_balance -> Text,
        pool_position -> Nullable<Text>,
        lp_token_value -> Nullable<Text>,
        lp_token_value_usd -> Nullable<Double>,
        underlying_asset_price -> Nullable<Double>,
        block_number -> Int8,
        lp_address -> Text,
    }
}

diesel::table! {
    options_volatility (option_address, block_number) {
        option_address -> Text,
        block_number -> Int8,
        volatility -> Nullable<Text>,
        option_position -> Nullable<Text>,
    }
}

diesel::table! {
    oracle_prices (id) {
        id -> Text,
        token_pair -> Text,
        price -> Int8,
        decimals -> SmallInt,
        last_updated_timestamp -> Int8,
        num_sources_aggregated -> SmallInt,
        oracle_name -> Text,
        block_number -> Int8,
    }
}

diesel::table! {
    starkscan_events (id) {
        id -> Text,
        block_hash -> Text,
        block_number -> Int8,
        transaction_hash -> Text,
        event_index -> Int8,
        from_address -> Text,
        keys -> Array<Text>,
        data -> Array<Text>,
        timestamp -> Int8,
        key_name -> Text,
    }
}

diesel::table! {
    referral_codes (wallet_address) {
        wallet_address -> Text,
        referral_code -> Text,
    }
}

diesel::table! {
    referral_events (id) {
        id -> Int4,
        referred_wallet_address -> Text,
        referral_code -> Text,
        source -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    insurance_events (id) {
        id -> Int4,
        user_address -> Text,
        calldata -> Array<Text>,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    user_points (id) {
        id -> Int4,
        user_address -> Text,
        timestamp -> Timestamp,
        trading_points -> Int8,
        liquidity_points -> Int8,
        referral_points -> Int8,
        vote_points -> Int8,
    }
}

diesel::table! {
    braavos_bonus (user_address) {
        user_address -> Text,
        pro_score_80 -> Nullable<Int8>,
        braavos_referral -> Nullable<Int8>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    events,
    options,
    blocks,
    pool_state,
    pools,
    options_volatility,
    oracle_prices,
    starkscan_events,
);

diesel::allow_tables_to_appear_in_same_query!(referral_codes, referral_events,);

diesel::joinable!(pool_state -> blocks (block_number));
diesel::joinable!(options_volatility -> blocks (block_number));
diesel::joinable!(oracle_prices -> blocks (block_number));
diesel::joinable!(referral_events -> referral_codes (referral_code));
