CREATE TABLE referral_events (
  id SERIAL PRIMARY KEY,
  referred_wallet_address TEXT NOT NULL,
  referral_code TEXT NOT NULL,
  timestamp TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY (referral_code) REFERENCES referral_codes(referral_code)
);

CREATE TABLE referral_codes (
  wallet_address TEXT PRIMARY KEY,
  referral_code TEXT NOT NULL UNIQUE
);