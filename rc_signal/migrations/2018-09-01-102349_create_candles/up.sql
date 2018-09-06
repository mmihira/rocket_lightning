CREATE TABLE candles (
  period INT NOT NULL,
  start_time BIGINT NOT NULL,
  end_time BIGINT NOT NULL,
  open REAL NOT NULL,
  close REAL NOT NULL,
  high REAL NOT NULL,
  low REAL NOT NULL,
  vol REAL NOT NULL,
  rsi REAL NOT NULL,
  sma_9 REAL NOT NULL,
  sma_12 REAL NOT NULL,
  sma_26 REAL NOT NULL,
  ema_9 REAL NOT NULL,
  ema_12 REAL NOT NULL,
  ema_26 REAL NOT NULL,
  PRIMARY KEY(period, start_time, end_time),
  FOREIGN KEY (period) REFERENCES candle_period (id)
)
