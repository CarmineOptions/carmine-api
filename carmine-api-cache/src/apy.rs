use carmine_api_core::{
    network::NEW_AMM_GENESIS_TIMESTAMP,
    types::{PoolStateWithTimestamp, APY},
};

const WEEK_SECS: i64 = 604800;
const DAY_SECS: i64 = 86400;
const YEAR_SECONDS: i64 = 31536000;

fn median(numbers: &mut Vec<i64>) -> f64 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid] as f64
}

fn to_percentage(n: f64) -> f64 {
    (n - 1.0) * 100.0
}

pub fn calculate_apy(state: &Vec<PoolStateWithTimestamp>) -> APY {
    if state.len() < 10000 {
        // cannot calculate for less than a week of data
        return APY {
            week: 0.0,
            week_annualized: 0.0,
            launch: 0.0,
            launch_annualized: 0.0,
        };
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

    let last_day_median = median(&mut last_day);
    let week_ago_median = median(&mut week_ago);

    let week = last_day_median / week_ago_median as f64;
    let week_annualized = week.powi(52);

    let seconds_since_launch = now - NEW_AMM_GENESIS_TIMESTAMP;
    let year_fraction = YEAR_SECONDS as f64 / seconds_since_launch as f64;

    let launch = last_day_median / 10_f64.powf(18.0);
    let launch_annualized = launch.powf(year_fraction);

    APY {
        week: to_percentage(week),
        week_annualized: to_percentage(week_annualized),
        launch: to_percentage(launch),
        launch_annualized: to_percentage(launch_annualized),
    }
}
