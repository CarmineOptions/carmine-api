use std::env;

use carmine_api_core::network::Network;
use carmine_api_db::get_options_volatility;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("ENVIRONMENT", "docker");
    env::set_var("DB_IP", "34.159.91.62");

    let v = get_options_volatility(&Network::Mainnet);

    println!("{}", v.len());
}
