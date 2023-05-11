use std::io::prelude::*;
use std::{fs::File, time::Instant};

use carmine_api_core::network::{call_lp_address, Network};
use carmine_api_db::{get_options_volatility, get_pool_state};
use dotenvy::dotenv;
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let network = Network::Mainnet;

    let now = Instant::now();
    let state = get_pool_state(call_lp_address(&network), &network);
    println!("Reading state took {:.2?}", now.elapsed());

    let now = Instant::now();
    let opt_vol = get_options_volatility(&network);
    println!("Reading options took {:.2?}", now.elapsed());

    let mut file = File::create("state.json").unwrap();
    file.write(to_string_pretty(&state).unwrap().as_bytes())
        .unwrap();

    let mut file2 = File::create("options_with_volatility.json").unwrap();
    file2
        .write(to_string_pretty(&opt_vol).unwrap().as_bytes())
        .unwrap();
}
