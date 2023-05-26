use carmine_api_core::types::PoolStateWithTimestamp;

const WEEK_SECS: i64 = 604800;
const DAY_SECS: i64 = 86400;

fn average(numbers: &mut Vec<i64>) -> f64 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid] as f64
}

pub fn calculate_apy(state: &Vec<PoolStateWithTimestamp>) -> f64 {
    if state.len() < 10000 {
        // cannot calculate for less than a week of data
        return 0.0;
    }
    let now = state
        .iter()
        .max_by_key(|state| state.timestamp)
        .unwrap()
        .timestamp;
    let mut last_day: Vec<i64> = state
        .into_iter()
        .filter(|v| v.timestamp > now - DAY_SECS)
        .filter_map(|v| v.lp_token_value.as_ref())
        .filter_map(|v| i64::from_str_radix(v.trim_start_matches("0x"), 16).ok())
        .collect();
    let mut week_ago: Vec<i64> = state
        .into_iter()
        .filter(|v| v.timestamp < now - WEEK_SECS && v.timestamp > now - WEEK_SECS - DAY_SECS)
        .filter_map(|v| v.lp_token_value.as_ref())
        .filter_map(|v| i64::from_str_radix(v.trim_start_matches("0x"), 16).ok())
        .collect();

    let last_day_average = average(&mut last_day);
    let week_ago_average = average(&mut week_ago);

    let wpy = last_day_average / week_ago_average as f64;
    let apy = wpy.powi(52);
    (apy - 1.0) * 100 as f64 // actual gain in percentage
}
