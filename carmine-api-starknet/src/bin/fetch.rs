use carmine_api_core::network::{Network, Protocol};
use carmine_api_starknet::starkscan::{api_call_text, api_url};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = api_url(&Network::Mainnet, &Protocol::ZkLend);

    match api_call_text(url.as_str()).await {
        Ok(res) => println!("SUCCEEDED\n{:?}", res),
        Err(e) => println!("FAILED\n{:?}", e),
    }
}
