use carmine_api_cache::Cache;
use carmine_api_core::network::Network;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cache = Cache::new(Network::Mainnet).await;

    let app_data = cache.get_app_data();

    // let eth_usdc_call_state = app_data.state.get("eth-usdc-call").unwrap().first();
    // let eth_usdc_put_state = app_data.state.get("eth-usdc-put").unwrap().first();
    // let btc_usdc_call_state = app_data.state.get("btc-usdc-call").unwrap().first();
    // let btc_usdc_put_state = app_data.state.get("btc-usdc-put").unwrap().first();

    // println!("STATE:");
    // println!("eth-usdc-call: {:#?}", eth_usdc_call_state);
    // println!("eth-usdc-put: {:#?}", eth_usdc_put_state);
    // println!("btc-usdc-call: {:#?}", btc_usdc_call_state);
    // println!("btc-usdc-put: {:#?}", btc_usdc_put_state);

    println!("APY: {:#?}", app_data.apy);
}
