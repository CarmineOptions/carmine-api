CREATE TABLE user_points (
  id SERIAL PRIMARY KEY,
  user_address TEXT NOT NULL,
  timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  trading_points Int8 NOT NULL,
  liquidity_points Int8 NOT NULL,
  referral_points Int8 NOT NULL
);
