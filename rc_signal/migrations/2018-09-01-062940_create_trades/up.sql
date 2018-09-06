CREATE TABLE trades (
  id SERIAL PRIMARY KEY,
  tid SERIAL UNIQUE,
  timestamp BIGINT NOT NULL,
  vol REAL NOT NULL,
  price REAL NOT NULL
);
CREATE INDEX trades_timestamp ON trades (timestamp);
