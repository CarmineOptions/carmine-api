CREATE TABLE braavos_bonus (
  user_address TEXT NOT NULL PRIMARY KEY,
  -- timestamp when reached 80, NULL if bellow 80
  pro_score_80 Int8,
  -- timestamp when referred by braavos, NULL if not referred by braavos
  braavos_referral Int8
);