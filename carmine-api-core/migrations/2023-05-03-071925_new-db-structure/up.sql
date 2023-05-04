CREATE TABLE blocks (
  block_number Int8 NOT NULL,
  timestamp Int8 NOT NULL,
  PRIMARY KEY (block_number)
);

CREATE TABLE pools (
  lp_address text NOT NULL,
  PRIMARY KEY (lp_address)
);

CREATE TABLE options_volatility (
  volatility text NOT NULL,
  option_address text NOT NULL,
  block_number int NOT NULL,
  PRIMARY KEY (option_address, block_number),
  FOREIGN KEY (option_address) REFERENCES options (option_address),
  FOREIGN KEY (block_number) REFERENCES blocks (block_number)
);

CREATE TABLE pool_state (
  unlocked_cap text NOT NULL,
  locked_cap text NOT NULL,
  lp_balance text NOT NULL,
  pool_position text NOT NULL,
  lp_token_value text NOT NULL,
  block_number int NOT NULL,
  lp_address text NOT NULL,
  PRIMARY KEY (lp_address, block_number),
  FOREIGN KEY (block_number) REFERENCES blocks (block_number),
  FOREIGN KEY (lp_address) REFERENCES pools (lp_address)
);

ALTER TABLE
  options
ADD
  COLUMN lp_address text;

ALTER TABLE
  options
ADD
  CONSTRAINT fk_options_lp_pools FOREIGN KEY (lp_address) REFERENCES pools (lp_address);