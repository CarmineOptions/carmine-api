use std::time::{SystemTime, UNIX_EPOCH};

use reqwest::Error;
use serde::Deserialize;

use carmine_api_db::{
    get_braavos_eligible_user_addresses, get_braavos_users_proscore_80, upsert_braavos_pro_score_80,
};

#[derive(Deserialize, Debug)]
pub struct Score {
    pub score: u8,
}

pub async fn get_braavos_proscore(address: &str) -> Result<u8, Error> {
    let url = format!(
        "https://activity-api.braavos.app/pro-score?network=mainnet-alpha&account_address={}",
        address
    );
    let score = fetch_score(url.as_str()).await?;
    Ok(score.score)
}

pub async fn set_braavos_proscore(address: &str, ts: i64) -> Result<bool, Error> {
    let score = get_braavos_proscore(address).await?;
    if score >= 80 {
        match upsert_braavos_pro_score_80(address, ts) {
            Ok(_) => {
                println!("Updated {}", address);
                return Ok(true);
            }
            Err(e) => println!("Failed updating {}, {:#?}", address, e),
        };
    }
    Ok(false)
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

    let mut count: u16 = 0;

    for user in eligible.iter() {
        let res = set_braavos_proscore(user, ts).await;

        if let Ok(updated) = res {
            if updated {
                count += 1;
            }
        }
    }

    println!("Updated proscore for {} users", count);
}

async fn fetch_score(url: &str) -> Result<Score, Error> {
    let response = reqwest::get(url).await?.json::<Score>().await?;
    Ok(response)
}
