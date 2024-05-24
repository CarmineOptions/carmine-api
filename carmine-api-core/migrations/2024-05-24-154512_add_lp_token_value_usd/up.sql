ALTER TABLE
  pool_state
ADD
  COLUMN lp_token_value_usd DOUBLE PRECISION,
ADD
  COLUMN underlying_asset_price DOUBLE PRECISION;