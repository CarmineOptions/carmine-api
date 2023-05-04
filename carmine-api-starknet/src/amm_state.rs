use std::time::{Duration, Instant};

use carmine_api_core::{network::Network, types::DbBlock};
use carmine_api_db::{
    create_batch_of_pool_states, create_batch_of_volatilities, create_block, get_last_block_in_db,
};
use starknet::core::types::BlockId;
use tokio::{join, time::sleep};

use crate::carmine::Carmine;

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

    async fn update_single_block(&self, block_number: i64) -> Result<(), ()> {
        let strk_block = match self
            .carmine
            .get_block_by_id(BlockId::Number(block_number as u64))
            .await
        {
            Ok(v) => v,
            Err(_) => return Err(()),
        };
        let block = DbBlock {
            block_number: i64::try_from(strk_block.block_number.unwrap()).unwrap(),
            timestamp: i64::try_from(strk_block.timestamp).unwrap(),
        };

        let (options_volatility_result, amm_state_result) = join!(
            self.carmine.get_all_options_volatility(&block),
            self.carmine.get_amm_state(&block)
        );

        match (options_volatility_result, amm_state_result) {
            (Ok(options_volatility), Ok(amm_state)) => {
                // got everything - store it to the database
                create_block(&block, &self.network);
                create_batch_of_volatilities(&options_volatility, &self.network);
                create_batch_of_pool_states(&amm_state, &self.network);
                Ok(())
            }
            _ => Err(()),
        }
    }

    pub async fn update_state(&self) {
        let last_block_db = get_last_block_in_db(&self.network);
        let last_block_starknet = self
            .carmine
            .get_latest_block()
            .await
            .expect("Failed getting latest block from starknet");

        let start = last_block_db.block_number + 1;
        let finish = i64::try_from(last_block_starknet.block_number.unwrap()).unwrap();

        // do nothing if up to date
        if start < finish {
            self.update_state_over_block_range(start, finish, 1).await;
        }
    }

    pub async fn update_state_over_block_range(&self, start: i64, finish: i64, increment: i64) {
        println!("getting data from block #{} to #{}", start, finish);

        let mut n = start;

        while n <= finish {
            let now = Instant::now();
            match self.update_single_block(n).await {
                Ok(_) => {
                    println!("Updated block #{} in {:.2?}", n, now.elapsed());
                    // only increment if successfull
                    n = n + increment;
                }
                Err(_) => {
                    println!(
                        "Failed updating block #{} in {:.2?}, retrying...",
                        n,
                        now.elapsed()
                    );
                    // error is most likely rate limit
                    // wait 3s to be able to fetch again
                    sleep(Duration::from_secs(3)).await;
                }
            }
        }

        println!("State updated");
    }
}
