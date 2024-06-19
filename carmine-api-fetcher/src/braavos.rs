use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

use carmine_api_db::{
    get_braavos_eligible_user_addresses, get_braavos_users_proscore_80,
    get_first_braavos_referrals, upsert_braavos_pro_score_80, upsert_braavos_referral,
};

#[derive(Serialize)]
struct RequestBody {
    #[serde(rename = "mainnet-alpha")]
    mainnet_alpha: Vec<String>,
}

#[derive(Deserialize)]
struct ResponseBody {
    #[serde(rename = "mainnet-alpha")]
    mainnet_alpha: HashMap<String, Score>,
}

#[derive(Deserialize, Debug)]
pub struct Score {
    pub score: u8,
}

async fn get_braavos_proscore(
    addresses: Vec<String>,
) -> Result<HashMap<String, u8>, reqwest::Error> {
    let client = Client::new();
    let url = "https://activity-api.braavos.app/pro-score";
    let request_body = RequestBody {
        mainnet_alpha: addresses,
    };

    let response = client
        .post(url)
        .json(&request_body)
        .send()
        .await?
        .json::<ResponseBody>()
        .await?;

    let transformed_response = response
        .mainnet_alpha
        .into_iter()
        .map(|(k, v)| (k, v.score))
        .collect::<HashMap<String, u8>>();

    Ok(transformed_response)
}

pub async fn set_braavos_proscore(addresses: Vec<String>, ts: i64) -> Result<usize, Error> {
    let scores = get_braavos_proscore(addresses).await?;

    let mut updated = 0;

    for (address, score) in scores {
        if score >= 80 {
            match upsert_braavos_pro_score_80(address.as_str(), ts) {
                Ok(_) => updated += 1,
                Err(_) => (),
            }
        }
    }

    Ok(updated)
}

pub fn update_braavos_referrals() {
    let referrals = get_first_braavos_referrals().expect("Failed getting braavos referrals");

    for (ts, s) in referrals {
        let _ = upsert_braavos_referral(s.as_str(), ts);
    }

    println!("Updated Braavos referrals");
}

pub async fn update_braavos_proscore() {
    let mut eligible = get_braavos_eligible_user_addresses();
    let proscore_users = get_braavos_users_proscore_80();

    println!("All eligible: {}", eligible.len());
    println!("Pros: {}", proscore_users.len());

    // Filter eligible to remove those that are already updated
    eligible.retain(|user| !proscore_users.contains(user));

    println!("To be fetched: {}", eligible.len());

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ts = since_the_epoch.as_secs() as i64;

    let mut count: usize = 0;

    for chunk in eligible.chunks(100) {
        let chunk_vec = chunk.to_vec();
        let res = set_braavos_proscore(chunk_vec, ts).await;

        if let Ok(updated) = res {
            count += updated;
        }
    }

    println!("Updated proscore for {} users", count);
}
