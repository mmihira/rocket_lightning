table! {
    candle_period (id) {
        id -> Int4,
        period_name -> Varchar,
    }
}

table! {
    candles (period, start_time, end_time) {
        period -> Int4,
        start_time -> Int8,
        end_time -> Int8,
        open -> Float4,
        close -> Float4,
        high -> Float4,
        low -> Float4,
        vol -> Float4,
        rsi -> Float4,
        sma_9 -> Float4,
        sma_12 -> Float4,
        sma_26 -> Float4,
        ema_9 -> Float4,
        ema_12 -> Float4,
        ema_26 -> Float4,
    }
}

table! {
    trades (id) {
        id -> Int4,
        tid -> Int4,
        timestamp -> Int8,
        vol -> Float4,
        price -> Float4,
    }
}

joinable!(candles -> candle_period (period));

allow_tables_to_appear_in_same_query!(
    candle_period,
    candles,
    trades,
);
