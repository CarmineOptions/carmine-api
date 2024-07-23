use std::collections::HashMap;

use carmine_api_core::types::{DbBlock, OraclePriceConcise};
use carmine_api_db::get_blocks_since_new_amm;

pub struct HistoricalPrices {
    prices: HashMap<String, HashMap<i64, f32>>,
    blocks: Vec<DbBlock>,
}

pub enum BlockId {
    Timestamp(i64),
    BlockNumber(i64),
}

impl HistoricalPrices {
    pub fn new(oracle_prices: &HashMap<String, Vec<OraclePriceConcise>>) -> Self {
        let mut nested_map: HashMap<String, HashMap<i64, f32>> = HashMap::new();

        for (token_pair, prices) in oracle_prices {
            println!("{}", token_pair);
            let mut inner_map: HashMap<i64, f32> = HashMap::new();
            for price in prices {
                let numeric_price = price.price as f32 / 10f32.powi(price.decimals as i32);
                inner_map.insert(price.block_number, numeric_price);
            }
            nested_map.insert(token_pair.to_string(), inner_map);
        }

        let mut blocks = get_blocks_since_new_amm();
        blocks.sort_by_key(|block| block.timestamp);

        HistoricalPrices {
            prices: nested_map,
            blocks,
        }
    }

    fn get_block_number_from_timestamp(&self, ts: i64) -> i64 {
        // Try to find the exact match first
        if let Some(block) = self.blocks.iter().find(|&block| block.timestamp == ts) {
            return block.block_number;
        }

        // If no exact match, find the closest timestamp less than the target_timestamp
        let nearest_less_than = self
            .blocks
            .iter()
            .filter(|&block| block.timestamp < ts)
            .max_by_key(|&block| block.timestamp)
            .map(|block| block.block_number);

        nearest_less_than.expect("Failed to find block number for timestamp")
    }

    pub fn get_price(&self, pool_id: &str, block_id: BlockId) -> f32 {
        let usdc_price = 1.0;
        let pair: &str = match pool_id {
            "eth-usdc-call" => "eth-usdc",
            "eth-usdc-put" => return usdc_price,
            "btc-usdc-call" => "btc-usdc",
            "btc-usdc-put" => return usdc_price,
            "eth-strk-call" => "eth-usdc",
            "eth-strk-put" => "strk-usdc",
            "strk-usdc-call" => "strk-usdc",
            "strk-usdc-put" => return usdc_price,
            _ => unreachable!("invalid pool id"),
        };
        let pair_map = match self.prices.get(pair) {
            Some(map) => map,
            None => panic!("Invalid token_pair"),
        };

        let block_number: i64 = match block_id {
            BlockId::Timestamp(n) => self.get_block_number_from_timestamp(n),
            BlockId::BlockNumber(n) => n,
        };

        if let Some(price) = pair_map.get(&block_number) {
            return price.clone();
        }

        match pair_map
            .iter()
            .filter(|(&block, _)| block <= block_number)
            .max_by_key(|(&block, _)| block)
            .map(|(_, &price)| price)
        {
            Some(nearest_price) => nearest_price,
            None => panic!("Could not find price"),
        }
    }
}
