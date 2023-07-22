CREATE TABLE variations (
  id BIGSERIAL PRIMARY KEY,
  
  piercer      BOOLEAN NOT NULL,
  marskman     BOOLEAN NOT NULL,
  sharpshooter BOOLEAN NOT NULL,
  
  core_eject   BOOLEAN NOT NULL,
  pump_charge  BOOLEAN NOT NULL,

  attractor    BOOLEAN NOT NULL,
  overheat     BOOLEAN NOT NULL,

  electric     BOOLEAN NOT NULL,
  malicious    BOOLEAN NOT NULL,
  drill        BOOLEAN NOT NULL,

  freezeframe  BOOLEAN NOT NULL,
  srs_cannon   BOOLEAN NOT NULL
);

CREATE TABLE scores (
  id BIGSERIAL PRIMARY KEY,
  variation BIGSERIAL REFERENCES variations(id) ON DELETE CASCADE,

  steam_id BIGINT UNIQUE NOT NULL,

  score INT NOT NULL,
  progress INT NOT NULL,

  UNIQUE(variation, steam_id)
);
