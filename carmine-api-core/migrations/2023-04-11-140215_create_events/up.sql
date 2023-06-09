CREATE TABLE events (
    block_hash              Text NOT NULL,
    block_number            Int8 NOT NULL,
    transaction_hash        Text NOT NULL,
    event_index             Int8 NOT NULL,
    from_address            Text NOT NULL,
    timestamp               Int8 NOT NULL,
    action                  Text NOT NULL,
    caller                  Text NOT NULL,
    token_address           Text NOT NULL,
    capital_transfered      Text NOT NULL,
    tokens_minted           Text NOT NULL,
    PRIMARY KEY (transaction_hash)
);