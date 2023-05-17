ALTER TABLE
  options_volatility
ADD
  COLUMN option_position text;

ALTER TABLE
  options_volatility
ALTER COLUMN
  volatility DROP NOT NULL;