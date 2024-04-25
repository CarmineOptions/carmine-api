use carmine_api_cache::defispring::get_defispring_stats;

use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let info = get_defispring_stats().await;

    println!("{:#?}", info);
}
