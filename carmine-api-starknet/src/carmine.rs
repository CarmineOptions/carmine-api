use carmine_api_core::network::{amm_address, Network, TESTNET_CONTRACT_ADDRESS};
use carmine_api_core::pool::{get_all_pool_addresses, get_all_pools, Pool};
use carmine_api_core::types::{DbBlock, IOption, OptionVolatility, PoolState};
use carmine_api_db::{create_batch_of_options, get_option_with_address, get_options, get_pools};
use carmine_api_rpc_gateway::{call, carmine_get_block_header, BlockTag, Entrypoint, RpcError};
use futures::future::join_all;
use futures::FutureExt;
use starknet::core::types::FieldElement;
use starknet::{self};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tokio::try_join;

const TWO_DAYS_SECS: i64 = 172800;

#[allow(dead_code)]
fn to_hex(v: FieldElement) -> String {
    format!("{:#x}", v)
}

const TEN_POW_18: &'static str = "0xde0b6b3a7640000";

#[allow(dead_code)]
struct FunctionDescriptor<'a> {
    name: &'a str,
    selector: FieldElement,
}

pub struct Carmine {
    pools: Vec<Pool>,
    network: Network,
}

impl Carmine {
    pub fn new(network: Network) -> Self {
        Carmine {
            network,
            pools: get_all_pools(&network),
        }
    }

    pub async fn amm_call(
        &self,
        entry_point_selector: String,
        calldata: Vec<String>,
        block: BlockTag,
    ) -> Result<Vec<String>, RpcError> {
        let contract_address = amm_address(&self.network).to_string();
        call(
            contract_address,
            entry_point_selector,
            calldata,
            block,
            &self.network,
        )
        .await
    }

    pub async fn get_all_non_expired_options_with_premia(&self) -> Result<Vec<String>, RpcError> {
        let pool_addresses = get_all_pool_addresses(&self.network);

        let mut futures = vec![];

        match self.network {
            Network::Mainnet => {
                for address in pool_addresses {
                    futures.push(call(
                        "0x01569044e6ce80e6c9f777ee24aa8dafd77d5b671ab623c92667bf8ca488bc4f"
                            .to_string(), // aux contract to bypass BTC option problem
                        "0x28465ebd72d95a0985251c1cbd769fd70bd499003d1ed138cc4263dcd4154a8"
                            .to_string(),
                        vec![address.to_string()],
                        BlockTag::Latest,
                        &self.network,
                    ));
                }
            }
            Network::Testnet => {
                for address in pool_addresses {
                    futures.push(call(
                        TESTNET_CONTRACT_ADDRESS.to_string(), // AMM for testnet
                        "0x28465ebd72d95a0985251c1cbd769fd70bd499003d1ed138cc4263dcd4154a8"
                            .to_string(),
                        vec![address.to_string()],
                        BlockTag::Latest,
                        &self.network,
                    ));
                }
            }
        }

        let call_results = join_all(futures).await;

        let mut option_data = vec![];

        for result in call_results {
            match result {
                Err(e) => return Err(e),
                Ok(mut v) => {
                    v.remove(0); // first element specifies length - remove it
                    option_data.extend(v);
                }
            }
        }

        Ok(option_data)
    }

    pub async fn get_option_info_from_addresses(
        &self,
        option_address: &str,
    ) -> Result<IOption, &str> {
        let pool_addresses: Vec<String> = self
            .pools
            .iter()
            .map(|pool| pool.address.to_string())
            .collect();

        let mut futures = vec![];

        for address in &pool_addresses {
            futures.push(self.amm_call(
                format!("{}", Entrypoint::GetOptionInfoFromAddress),
                vec![address.to_string(), option_address.to_string()],
                BlockTag::Latest,
            ))
        }

        let contract_results = join_all(futures).await;

        for (i, result) in contract_results.iter().enumerate() {
            if let Ok(data) = result {
                assert_eq!(data.len(), 6, "Got wrong size Option result");

                let option_side = data[0].parse::<i16>().expect("Failed to parse side");
                let option_type = data[5].parse::<i16>().expect("Failed to parse type");
                let maturity = data[1].parse::<i64>().expect("Failed to parse maturity");
                let strike_price = data[2].to_owned();
                let quote_token_address = data[3].to_owned();
                let base_token_address = data[4].to_owned();
                let lp_address = pool_addresses[i].to_owned();

                return Ok(IOption {
                    option_side,
                    option_type,
                    strike_price,
                    maturity,
                    quote_token_address,
                    base_token_address,
                    option_address: String::from(option_address),
                    lp_address,
                });
            }
        }

        Err("Failed to find option with given address")
    }

    pub async fn get_option_token_address(
        &self,
        lptoken_address: &String,
        option_side: String,
        maturity: String,
        strike_price: String,
    ) -> Result<String, &str> {
        let calldata = vec![
            lptoken_address.to_owned(),
            option_side,
            maturity,
            strike_price,
            "0".to_string(), // zero for strike_price: cubit::f128::types::fixed::Fixed
        ];

        match self
            .amm_call(
                format!("{}", Entrypoint::GetOptionTokenAddress),
                calldata,
                BlockTag::Latest,
            )
            .await
        {
            Ok(data) => Ok(data[0].to_owned()),
            Err(e) => {
                println!("Failed \"get_option_token_address\" \n{:#?}", e);
                Err("Failed \"get_option_token_address\"")
            }
        }
    }

    async fn get_options_with_addresses_from_single_pool(&self, pool_address: &String) {
        let contract_result = self
            .amm_call(
                format!("{}", Entrypoint::GetAllOptions),
                vec![pool_address.to_owned()],
                BlockTag::Latest,
            )
            .await;

        let data: Vec<String> = match contract_result {
            Err(provider_error) => {
                println!("{:?}", provider_error);
                return;
            }
            Ok(v) => {
                let mut res = v;
                // first element is length of result array - remove it
                res.remove(0);

                res
            }
        };

        let option_length = match self.network {
            Network::Mainnet => 6,
            // TODO: C1 has longer options - change when mainnet is also C1
            Network::Testnet => 7,
        };

        // each option has 6 fields
        let chunks = data.chunks(option_length);

        let mut options: Vec<IOption> = vec![];

        let mut cache_hit = 0;
        let mut fetched = 0;

        for option_vec in chunks {
            if option_vec.len() != option_length {
                println!("Wrong option_vec size!");
                continue;
            }

            let option_side =
                i16::from_str_radix(&option_vec[0][2..], 16).expect("Failed to parse side");
            let maturity =
                i64::from_str_radix(&option_vec[1][2..], 16).expect("Failed to parse maturity");
            let strike_price = option_vec[2].to_owned();
            let lp_address = pool_address.to_owned();

            let db_hit = get_option_with_address(
                &self.network,
                option_side,
                maturity,
                &strike_price,
                &lp_address,
            );

            if let Some(option_with_address) = db_hit {
                options.push(option_with_address);
                cache_hit += 1;
                continue;
            }

            let (type_index, base_index, quote_index) = match self.network {
                Network::Mainnet => (5, 4, 3),
                // TODO: C1 has longer options - change when mainnet is also C1
                Network::Testnet => (6, 5, 4),
            };

            // this part only runs if option not already in the DB
            let option_type = i16::from_str_radix(&option_vec[type_index][2..], 16)
                .expect("Failed to parse type");
            let quote_token_address = option_vec[quote_index].to_owned();
            let base_token_address = option_vec[base_index].to_owned();

            // avoid running into rate limit starknet error
            sleep(Duration::from_secs(2)).await;

            let option_address_result = self
                .get_option_token_address(
                    pool_address,
                    option_vec[0].to_owned(),
                    option_vec[1].to_owned(),
                    option_vec[2].to_owned(),
                )
                .await;

            let option_address = match option_address_result {
                Err(e) => {
                    println!("Failed to get option address\n{}", e);
                    continue;
                }
                Ok(v) => v.to_lowercase(),
            };

            let option = IOption {
                option_side,
                maturity,
                strike_price,
                quote_token_address,
                base_token_address,
                option_type,
                option_address,
                lp_address,
            };
            fetched += 1;
            options.push(option);
        }

        println!(
            "{} options from cache, {} options newly fetched",
            cache_hit, fetched
        );

        create_batch_of_options(&options, &self.network);
    }

    pub async fn get_options_with_addresses(&self) {
        let pool_addresses = get_all_pool_addresses(&self.network);

        for address in pool_addresses {
            self.get_options_with_addresses_from_single_pool(&address.to_string())
                .await;
        }
    }

    pub async fn get_all_lptoken_addresses(&self) -> Result<Vec<String>, ()> {
        let call_result = self
            .amm_call(
                format!("{}", Entrypoint::GetAllLPTokenAddresses),
                vec![],
                BlockTag::Latest,
            )
            .await;

        let mut data = match call_result {
            Ok(v) => v,
            _ => return Err(()),
        };

        if data.len() < 2 {
            return Err(());
        } else {
            // remove length
            data.remove(0);
        }

        Ok(data)
    }

    pub async fn get_pool_single_value(
        &self,
        block_number: i64,
        pool: String,
        entry_point: Entrypoint,
    ) -> Result<String, RpcError> {
        match self
            .amm_call(
                format!("{}", entry_point),
                vec![pool],
                BlockTag::Number(block_number),
            )
            .await
        {
            Ok(v) => Ok(v[0].to_owned()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_pool_locked_capital(
        &self,
        block_number: i64,
        pool: String,
    ) -> Result<String, RpcError> {
        self.get_pool_single_value(block_number, pool, Entrypoint::GetPoolLockedCapital)
            .await
    }

    pub async fn get_unlocked_capital(
        &self,
        block_number: i64,
        pool: String,
    ) -> Result<String, RpcError> {
        self.get_pool_single_value(block_number, pool, Entrypoint::GetUnlockedCapital)
            .await
    }

    pub async fn get_lpool_balance(
        &self,
        block_number: i64,
        pool: String,
    ) -> Result<String, RpcError> {
        self.get_pool_single_value(block_number, pool, Entrypoint::GetLpoolBalance)
            .await
    }

    pub async fn get_value_of_pool_position(
        &self,
        block_number: i64,
        pool: String,
    ) -> Result<Option<String>, RpcError> {
        match self
            .get_pool_single_value(block_number, pool, Entrypoint::GetValueOfPoolPosition)
            .await
        {
            Ok(v) => Ok(Some(v)),
            Err(e) if matches!(e, RpcError::ContractError(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn get_value_of_lp_token(
        &self,
        block_number: i64,
        pool: String,
    ) -> Result<Option<String>, RpcError> {
        match self
            .amm_call(
                format!("{}", Entrypoint::GetUnderlyingForLptoken),
                vec![pool, TEN_POW_18.to_owned(), "0".to_owned()],
                BlockTag::Number(block_number),
            )
            .await
        {
            Ok(v) => Ok(Some(v[0].to_owned())),
            Err(e) if matches!(e, RpcError::ContractError(_)) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn get_locked_unlocked_total_capital_for_pool(
        &self,
        pool: String,
        block_number: i64,
    ) -> Result<
        (
            String,
            String,
            String,
            Option<String>,
            Option<String>,
            String,
        ),
        RpcError,
    > {
        let now = Instant::now();
        let res = try_join!(
            self.get_pool_locked_capital(block_number, pool.to_owned()),
            self.get_unlocked_capital(block_number, pool.to_owned()),
            self.get_lpool_balance(block_number, pool.to_owned()),
            self.get_value_of_pool_position(block_number, pool.to_owned()),
            self.get_value_of_lp_token(block_number, pool.to_owned()),
        );
        println!(
            "Fetched pool state {} in {:.2?}",
            pool.get(0..10).unwrap(),
            now.elapsed()
        );
        match res {
            Ok((
                pool_locked_capital,
                unlocked_capital,
                lpool_balance,
                value_of_pool_position,
                value_of_lp_token,
            )) => Ok((
                pool_locked_capital,
                unlocked_capital,
                lpool_balance,
                value_of_pool_position,
                value_of_lp_token,
                pool,
            )),
            Err(e) => {
                println!("Failed getting balance data in block #{}", block_number);
                println!("{:#?}", e);
                Err(e)
            }
        }
    }

    pub async fn get_amm_state(&self, block: &DbBlock) -> Result<Vec<PoolState>, ()> {
        let pool_addresses: Vec<String> = get_pools(&self.network)
            .iter()
            .map(|p| p.lp_address.to_owned())
            .collect();

        let mut futures = vec![];

        for pool_address in pool_addresses {
            futures.push(
                self.get_locked_unlocked_total_capital_for_pool(pool_address, block.block_number)
                    .boxed(),
            );
        }

        let results = join_all(futures).await;

        let mut cumulative_state: Vec<PoolState> = vec![];

        for res in results {
            let (
                locked_cap,
                unlocked_cap,
                lpool_balance,
                value_pool_position,
                lp_token_value,
                pool_address,
            ) = match res {
                Ok(v) => v,
                _ => return Err(()),
            };

            cumulative_state.push(PoolState {
                unlocked_cap: unlocked_cap,
                locked_cap: locked_cap,
                lp_balance: lpool_balance,
                pool_position: match value_pool_position {
                    Some(v) => Some(v),
                    None => None,
                },
                lp_address: pool_address,
                block_number: block.block_number,
                lp_token_value: match lp_token_value {
                    Some(v) => Some(v),
                    None => None,
                },
            });
        }

        Ok(cumulative_state)
    }

    pub async fn get_all_options_volatility(
        &self,
        block: &DbBlock,
    ) -> Result<Vec<OptionVolatility>, ()> {
        let now = Instant::now();

        let options = get_options(&self.network);
        let mut to_store: Vec<OptionVolatility> = vec![];

        let mut non_expired_options: Vec<IOption> = vec![];
        for opt in options {
            // non expired
            if opt.maturity + TWO_DAYS_SECS > block.timestamp {
                non_expired_options.push(opt);

                // for expired set volatility to None
            }
            //  else {
            //     let option_volatility = OptionVolatility {
            //         block_number: block.block_number,
            //         option_address: opt.option_address,
            //         volatility: None,
            //         option_position: None,
            //     };
            //     to_store.push(option_volatility);
            // }
        }

        // volatility fails if too many requests, break it into chunks
        let chunk_size = 50;

        let vector_of_vectors: Vec<Vec<IOption>> = non_expired_options
            .chunks(chunk_size)
            .map(|chunk| chunk.to_vec()) // Convert each chunk to a vector
            .collect();

        for chunk in vector_of_vectors {
            let mut futures = vec![];

            for opt in chunk {
                futures.push(self.get_option_volatility(opt, block.block_number));
            }

            let results = join_all(futures).await;

            for res in results {
                let (volatility, option_position, option_address) = res;

                to_store.push(OptionVolatility {
                    block_number: block.block_number,
                    option_address,
                    volatility,
                    option_position,
                });
            }
        }
        println!("Options volatility fetched in {:.2?}", now.elapsed());
        Ok(to_store)
    }

    pub async fn get_option_volatility(
        &self,
        opt: IOption,
        block_number: i64,
    ) -> (Option<String>, Option<String>, String) {
        let lp_address = opt.lp_address;
        let maturity = format!("{:#x}", opt.maturity);
        let strike = opt.strike_price;
        let side = opt.option_side.to_string();

        let volatility_future = Box::pin(self.amm_call(
            format!("{}", Entrypoint::GetPoolVolatilityAuto),
            vec![
                lp_address.to_owned(),
                maturity.to_owned(),
                strike.to_owned(),
                "0".to_string(), // zero for strike_price: cubit::f128::types::fixed::Fixed
            ],
            BlockTag::Number(block_number),
        ));

        let position_future = Box::pin(self.amm_call(
            format!("{}", Entrypoint::GetOptionPosition),
            vec![lp_address, side, maturity, strike, "0".to_string()], // zero for strike_price: cubit::f128::types::fixed::Fixed
            BlockTag::Number(block_number),
        ));

        let mut results = join_all(vec![volatility_future, position_future]).await;

        let volatility_result = results.remove(0);
        let position_result = results.remove(0);

        let volatility: Option<String> = match volatility_result {
            Ok(v) => Some(v[0].to_owned()),
            Err(_) => None,
        };
        let position: Option<String> = match position_result {
            Ok(v) => Some(v[0].to_owned()),
            Err(_) => None,
        };

        (volatility, position, opt.option_address)
    }

    pub async fn get_block_by_id(&self, block_tag: BlockTag) -> Result<DbBlock, RpcError> {
        carmine_get_block_header(block_tag).await
    }

    pub async fn get_latest_block(&self) -> Result<DbBlock, RpcError> {
        self.get_block_by_id(BlockTag::Latest).await
    }
}
