CREATE TABLE starkscan_events (
  id Text NOT NULL,
  block_hash Text NOT NULL,
  block_number Int8 NOT NULL,
  transaction_hash Text NOT NULL,
  event_index Int8 NOT NULL,
  from_address Text,
  keys Text [],
  data Text [],
  timestamp Int8,
  key_name Text,
  PRIMARY KEY (id)
);