use carmine_api_core::{network::Network, types::DbBlock};
use carmine_api_db::update_token_value;
use carmine_api_starknet::carmine::Carmine;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let block_numbers: Vec<i64> = vec![389000, 388701, 389111, 388294];

    let c = Carmine::new(Network::Mainnet);

    for block_number in block_numbers {
        let block = DbBlock {
            block_number,
            // timestamp is not used
            timestamp: 0,
        };
        let amm_state_res = c.get_amm_state(&block).await;
        if let Ok(amm_states) = amm_state_res {
            for state in amm_states {
                if let Some(token_value) = state.lp_token_value {
                    update_token_value(
                        block_number,
                        state.lp_address,
                        token_value,
                        &Network::Mainnet,
                    );
                }
            }
        } else {
            println!("FAILED {}", block_number);
        }
    }
}
