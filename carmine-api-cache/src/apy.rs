use carmine_api_core::types::PoolStateWithTimestamp;

const WEEK_SECS: i64 = 604800;
const DAY_SECS: i64 = 86400;

fn average(numbers: &mut Vec<i64>) -> f64 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid] as f64
}

pub fn calculate_apy(state: &Vec<PoolStateWithTimestamp>) -> f64 {
    let mut index = match state.len() {
        n if n < 1000 => return 0.0,
        n => n - 1,
    };
    let mut last_day: Vec<i64> = vec![];
    let mut week_ago: Vec<i64> = vec![];

    let now = state[index].timestamp;
    index -= 1;

    loop {
        let current_state = &state[index];

        // last_day done
        if current_state.timestamp < now - DAY_SECS {
            break;
        }

        let lp_value_result = match &current_state.lp_token_value {
            Some(hex) => i64::from_str_radix(hex.trim_start_matches("0x"), 16),
            None => continue,
        };

        if let Ok(lp_value) = lp_value_result {
            last_day.push(lp_value);
        }
        index -= 1;
    }

    loop {
        let current_state = &state[index];

        // not yet week ago
        if current_state.timestamp > now - WEEK_SECS {
            index -= 1;
            continue;
        }

        // week_ago done
        if current_state.timestamp < now - WEEK_SECS - DAY_SECS {
            break;
        }

        let lp_value_result = match &current_state.lp_token_value {
            Some(hex) => i64::from_str_radix(hex.trim_start_matches("0x"), 16),
            None => continue,
        };

        if let Ok(lp_value) = lp_value_result {
            week_ago.push(lp_value);
        }
        index -= 1;
    }

    let last_day_average = average(&mut last_day);
    let week_ago_average = average(&mut week_ago);

    let wpy = last_day_average / week_ago_average as f64;
    let apy = wpy.powi(52);
    (apy - 1.0) * 100 as f64 // actual gain in percentage
}
