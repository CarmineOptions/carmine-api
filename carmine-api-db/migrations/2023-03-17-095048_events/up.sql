CREATE TABLE events (
    id                      serial NOT NULL,
    block_hash              Text NOT NULL,
    block_number            Int8 NOT NULL,
    transaction_hash        Text NOT NULL,
    event_index             Int8 NOT NULL,
    from_address            Text NOT NULL,
    timestamp               Int8 NOT NULL,
    action                  Text NOT NULL,
    caller                  Text NOT NULL,
    option_token            Text NOT NULL,
    capital_transfered      Text NOT NULL,
    option_tokens_minted    Text NOT NULL,
    CONSTRAINT events_pkey PRIMARY KEY (id),
    UNIQUE(transaction_hash)
)