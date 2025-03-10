ALTER TABLE
  referral_events
ADD
  COLUMN source text DEFAULT 'carmineoptions';

ALTER TABLE
  referral_events
ALTER COLUMN
  source
SET
  NOT NULL;