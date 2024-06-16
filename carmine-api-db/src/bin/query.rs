use carmine_api_core::network::{Network, LEGACY_AMM_CONTRACT_ADDRESS};
use carmine_api_db;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let braavos = carmine_api_db::get_braavos_users_proscore_80_with_timestamp();

    println!("{:#?}", braavos);
}
