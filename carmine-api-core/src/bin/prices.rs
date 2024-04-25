use carmine_api_core::utils::get_coingecko_prices;

#[tokio::main]
async fn main() {
    let res = get_coingecko_prices().await;

    println!("{:#?}", res);
}
