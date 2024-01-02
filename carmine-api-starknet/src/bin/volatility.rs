use carmine_api_core::{network::Network, types::IOption};
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let carmine = Carmine::new(Network::Mainnet);
    let option = IOption {
        option_side: 0,
        maturity: 1704412799,
        strike_price: String::from("0x1130000000000000000"),
        quote_token_address: String::from(
            "0x53c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
        ),
        base_token_address: String::from(
            "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
        ),
        option_type: 0,
        option_address: String::from(
            "0x9ea819ef37593eaa46f4edc3f246ae7c34cfd58dc9d106b505b225c8ec6aa1",
        ),
        lp_address: String::from(
            "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024",
        ),
    };
    let res = carmine.get_option_volatility(option, 491903).await;

    println!("{:#?}", res);
}
