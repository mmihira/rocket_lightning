use diesel::result::Error as DieselError;
use models::{Candle};
use analysis_range;

pub fn period_9<T: analysis_range::TimePeriod>(range: T) -> Vec<Result<Candle, DieselError>> {
    vec![]
}

fn for_period(period: i32) -> Vec<Result<Candle, DieselError>> {
    // let z = range.get_prev_period_range(period);
    vec![]
}
