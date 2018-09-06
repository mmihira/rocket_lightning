CREATE TABLE candle_period (
  id SERIAL PRIMARY KEY,
  period_name VARCHAR(100) UNIQUE NOT NULL
);
