use std::time::Instant;

use carmine_api_core::{network::Network, types::DbBlock};
use carmine_api_db::{
    create_batch_of_pool_states, create_batch_of_volatilities, create_block, get_last_block_in_db,
};
use starknet::core::types::BlockId;

use crate::carmine::Carmine;

const BLOCK_INCREMENT: i64 = 1;

pub struct AmmStateObserver {
    network: Network,
    carmine: Carmine,
}

impl AmmStateObserver {
    pub fn new(network: &Network) -> Self {
        AmmStateObserver {
            carmine: Carmine::new(*network),
            network: *network,
        }
    }

    async fn update_single_block(&self, block_number: i64) {
        let strk_block = self
            .carmine
            .get_block_by_id(BlockId::Number(block_number as u64))
            .await
            .expect("Failed unwrapping block");
        let block = DbBlock {
            block_number: i64::try_from(strk_block.block_number.unwrap()).unwrap(),
            timestamp: i64::try_from(strk_block.timestamp).unwrap(),
        };
        let options_volatility_result = self.carmine.get_all_options_volatility(&block).await;
        let amm_state_result = self.carmine.get_amm_state(&block).await;

        match (options_volatility_result, amm_state_result) {
            (Ok(options_volatility), Ok(amm_state)) => {
                // got everything - store it to the database
                create_block(&block, &self.network);
                create_batch_of_volatilities(&options_volatility, &self.network);
                create_batch_of_pool_states(&amm_state, &self.network);
            }
            _ => return,
        }
    }

    pub async fn update_state(&self) {
        let last_block_db = get_last_block_in_db(&self.network);
        let last_block_starknet = self
            .carmine
            .get_latest_block()
            .await
            .expect("Failed getting latest block from starknet");

        let start = last_block_db.block_number;
        let finish = i64::try_from(last_block_starknet.block_number.unwrap()).unwrap();

        self.update_state_over_block_range(start, finish).await;
    }

    pub async fn update_state_over_block_range(&self, start: i64, finish: i64) {
        println!("getting data from block #{} to #{}", start, finish);

        let mut n = start;

        while n < finish {
            println!("Updating data from block #{}", n);
            let now = Instant::now();
            self.update_single_block(n).await;
            println!("Updated in: {:.2?}", now.elapsed());
            n = n + BLOCK_INCREMENT;
        }

        println!("State updated");
    }
}
