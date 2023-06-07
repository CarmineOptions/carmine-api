CREATE TABLE oracle_prices (
  id Text NOT NULL,
  price Int8 NOT NULL,
  decimals SmallInt NOT NULL,
  last_updated_timestamp Int8 NOT NULL,
  num_sources_aggregated SmallInt,
  oracle_name Text NOT NULL,
  block_number Int8 NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (block_number) REFERENCES blocks (block_number)
);