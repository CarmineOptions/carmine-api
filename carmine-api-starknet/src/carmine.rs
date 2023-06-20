use carmine_api_core::network::{amm_address, call_lp_address, put_lp_address, Network};
use carmine_api_core::types::{DbBlock, IOption, OptionVolatility, PoolState};
use carmine_api_db::{create_batch_of_options, get_option_with_address, get_options, get_pools};
use futures::future::join_all;
use futures::FutureExt;
use starknet::core::types::{Block, CallContractResult, CallFunction, FieldElement};
use starknet::macros::selector;
use starknet::providers::{SequencerGatewayProvider, SequencerGatewayProviderError};
use starknet::{self, core::types::BlockId, providers::Provider};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;
use tokio::try_join;

const OPTION_NEAR_MATURITY: &'static str =
    "Unable to calculate position value, please wait till option with maturity";
const STALE_PRICE: &'static str = "Received price which is over an hour old";
const BLACK_SCHOLES: &'static str = "Black scholes function failed when calculating";
const TWO_DAYS_SECS: i64 = 172800;

fn format_call_contract_result(res: CallContractResult) -> Vec<String> {
    let mut arr: Vec<String> = vec![];

    // first element is length of the result - skip it
    for v in res.result.into_iter().skip(1) {
        let base_10 = format!("{}", v);
        arr.push(base_10);
    }

    arr
}

fn to_hex(v: FieldElement) -> String {
    format!("{:#x}", v)
}

const TEN_POW_18: &'static str = "1000000000000000000";

#[allow(dead_code)]
struct FunctionDescriptor<'a> {
    name: &'a str,
    selector: FieldElement,
}

pub struct Carmine {
    provider: SequencerGatewayProvider,
    amm_address: FieldElement,
    call_lp_address: FieldElement,
    put_lp_address: FieldElement,
    network: Network,
}

impl Carmine {
    pub fn new(network: Network) -> Self {
        let provider = match network {
            Network::Mainnet => SequencerGatewayProvider::starknet_alpha_mainnet(),
            Network::Testnet => SequencerGatewayProvider::starknet_alpha_goerli(),
        };

        let amm_address = FieldElement::from_hex_be(amm_address(&network)).unwrap();
        let call_lp_address = FieldElement::from_hex_be(call_lp_address(&network)).unwrap();
        let put_lp_address = FieldElement::from_hex_be(put_lp_address(&network)).unwrap();

        Carmine {
            provider,
            network,
            amm_address,
            call_lp_address,
            put_lp_address,
        }
    }

    pub async fn get_all_non_expired_options_with_premia(&self) -> Result<Vec<String>, ()> {
        let entrypoint = selector!("get_all_non_expired_options_with_premia");
        let call = self.provider.call_contract(
            CallFunction {
                contract_address: self.amm_address,
                entry_point_selector: entrypoint,
                calldata: vec![self.call_lp_address],
            },
            BlockId::Latest,
        );
        let put = self.provider.call_contract(
            CallFunction {
                contract_address: self.amm_address,
                entry_point_selector: entrypoint,
                calldata: vec![self.put_lp_address],
            },
            BlockId::Latest,
        );

        let contract_results = join_all(vec![call, put]).await;

        let mut fetched_data: Vec<String> = Vec::new();

        for result in contract_results {
            match result {
                Ok(v) => {
                    let mut formatted = format_call_contract_result(v);
                    fetched_data.append(&mut formatted);
                }
                Err(_) => {
                    println!("Failed fetching non-expired options");
                    return Err(());
                }
            }
        }
        Ok(fetched_data)
    }

    pub async fn get_option_info_from_addresses(
        &self,
        option_address: &str,
    ) -> Result<IOption, &str> {
        let entrypoint = selector!("get_option_info_from_addresses");
        let call = self.provider.call_contract(
            CallFunction {
                contract_address: self.amm_address,
                entry_point_selector: entrypoint,
                calldata: vec![
                    self.call_lp_address,
                    FieldElement::from_hex_be(option_address).unwrap(),
                ],
            },
            BlockId::Latest,
        );
        let put = self.provider.call_contract(
            CallFunction {
                contract_address: self.amm_address,
                entry_point_selector: entrypoint,
                calldata: vec![
                    self.put_lp_address,
                    FieldElement::from_hex_be(option_address).unwrap(),
                ],
            },
            BlockId::Latest,
        );

        let contract_results = join_all(vec![call, put]).await;

        for (i, result) in contract_results.into_iter().enumerate() {
            if let Ok(call_res) = result {
                let data = call_res.result;
                assert_eq!(data.len(), 6, "Got wrong size Option result");

                let option_side = format!("{}", data[0])
                    .parse::<i16>()
                    .expect("Failed to parse side");
                let option_type = format!("{}", data[5])
                    .parse::<i16>()
                    .expect("Failed to parse type");
                let maturity = format!("{}", data[1])
                    .parse::<i64>()
                    .expect("Failed to parse maturity");
                let strike_price = to_hex(data[2]);
                let quote_token_address = to_hex(data[3]);
                let base_token_address = to_hex(data[4]);
                let lp_address = match i {
                    0 => to_hex(self.call_lp_address),
                    1 => to_hex(self.put_lp_address),
                    _ => unreachable!("Hardcoded 2 lp_pools"),
                };

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
        lptoken_address: &FieldElement,
        option_side: FieldElement,
        maturity: FieldElement,
        strike_price: FieldElement,
    ) -> Result<String, &str> {
        let entrypoint = selector!("get_option_token_address");
        let contract_result = self
            .provider
            .call_contract(
                CallFunction {
                    contract_address: self.amm_address,
                    entry_point_selector: entrypoint,
                    calldata: vec![*lptoken_address, option_side, maturity, strike_price],
                },
                BlockId::Latest,
            )
            .await;

        match contract_result {
            Ok(v) => {
                let data = v.result[0];
                let address = to_hex(data);
                return Ok(address);
            }
            Err(e) => {
                println!("Failed \"get_option_token_address\" \n{}", e);
                return Err("Failed \"get_option_token_address\"");
            }
        }
    }

    async fn get_options_with_addresses_from_single_pool(&self, pool_address: &FieldElement) {
        let entrypoint = selector!("get_all_options");
        let contract_result = self
            .provider
            .call_contract(
                CallFunction {
                    contract_address: self.amm_address,
                    entry_point_selector: entrypoint,
                    calldata: vec![*pool_address],
                },
                BlockId::Latest,
            )
            .await;

        let data: Vec<FieldElement> = match contract_result {
            Err(provider_error) => {
                println!("{:?}", provider_error);
                return;
            }
            Ok(v) => {
                let mut res = v.result;
                // first element is length of result array - remove it
                res.remove(0);

                res
            }
        };

        // each option has 6 fields
        let chunks = data.chunks(6);

        let mut options: Vec<IOption> = vec![];

        let mut cache_hit = 0;
        let mut fetched = 0;

        for option_vec in chunks {
            if option_vec.len() != 6 {
                println!("Wrong option_vec size!");
                continue;
            }

            let option_side = format!("{}", option_vec[0])
                .parse::<i16>()
                .expect("Failed to parse side");
            let maturity = format!("{}", option_vec[1])
                .parse::<i64>()
                .expect("Failed to parse maturity");
            let strike_price = to_hex(option_vec[2]);
            let lp_address = to_hex(*pool_address);

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

            // this part only runs if option not already in the DB

            let option_type = format!("{}", option_vec[5])
                .parse::<i16>()
                .expect("Failed to parse type");
            let quote_token_address = to_hex(option_vec[3]);
            let base_token_address = to_hex(option_vec[4]);

            // avoid running into rate limit starknet error
            sleep(Duration::from_secs(2)).await;

            let option_address_result = self
                .get_option_token_address(pool_address, option_vec[0], option_vec[1], option_vec[2])
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

    /// This method fetches and stores in DB all options, addresses included.
    /// !This method is extremely slow, because it waits 2s between
    /// Starknet calls to avoid running into "rate limit" error!
    pub async fn get_options_with_addresses(&self) {
        self.get_options_with_addresses_from_single_pool(&self.call_lp_address)
            .await;
        self.get_options_with_addresses_from_single_pool(&self.put_lp_address)
            .await;
    }

    pub async fn get_all_lptoken_addresses(&self) -> Result<Vec<FieldElement>, ()> {
        let call_result = self
            .provider
            .call_contract(
                CallFunction {
                    contract_address: self.amm_address,
                    entry_point_selector: selector!("get_all_lptoken_addresses"),
                    calldata: vec![],
                },
                BlockId::Latest,
            )
            .await;

        let mut data = match call_result {
            Ok(v) => v.result,
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

    async fn amm_call(
        &self,
        block_number: i64,
        calldata: Vec<FieldElement>,
        function_descriptor: FunctionDescriptor<'_>,
    ) -> Result<FieldElement, starknet::providers::ProviderError<SequencerGatewayProviderError>>
    {
        match self
            .provider
            .call_contract(
                CallFunction {
                    contract_address: self.amm_address,
                    entry_point_selector: function_descriptor.selector,
                    calldata,
                },
                BlockId::Number(block_number as u64),
            )
            .await
        {
            Ok(call_result) => Ok(call_result.result[0]),
            Err(e) => Err(e),
        }
    }

    pub async fn get_pool_locked_capital(
        &self,
        block_number: i64,
        pool: FieldElement,
    ) -> Result<FieldElement, starknet::providers::ProviderError<SequencerGatewayProviderError>>
    {
        self.amm_call(
            block_number,
            vec![pool],
            FunctionDescriptor {
                name: "get_pool_locked_capital",
                selector: selector!("get_pool_locked_capital"),
            },
        )
        .await
    }

    pub async fn get_unlocked_capital(
        &self,
        block_number: i64,
        pool: FieldElement,
    ) -> Result<FieldElement, starknet::providers::ProviderError<SequencerGatewayProviderError>>
    {
        match self
            .amm_call(
                block_number,
                vec![pool],
                FunctionDescriptor {
                    name: "get_unlocked_capital",
                    selector: selector!("get_unlocked_capital"),
                },
            )
            .await
        {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }

    pub async fn get_lpool_balance(
        &self,
        block_number: i64,
        pool: FieldElement,
    ) -> Result<FieldElement, starknet::providers::ProviderError<SequencerGatewayProviderError>>
    {
        self.amm_call(
            block_number,
            vec![pool],
            FunctionDescriptor {
                name: "get_lpool_balance",
                selector: selector!("get_lpool_balance"),
            },
        )
        .await
    }

    pub async fn get_value_of_pool_position(
        &self,
        block_number: i64,
        pool: FieldElement,
    ) -> Result<
        Option<FieldElement>,
        starknet::providers::ProviderError<SequencerGatewayProviderError>,
    > {
        match self
            .amm_call(
                block_number,
                vec![pool],
                FunctionDescriptor {
                    name: "get_value_of_pool_position",
                    selector: selector!("get_value_of_pool_position"),
                },
            )
            .await
        {
            Ok(v) => Ok(Some(v)),
            Err(e) if e.to_string().contains(OPTION_NEAR_MATURITY) => {
                // this specific error message means that LP token value
                // cannot be calculated - return None
                Ok(None)
            }
            Err(e) if e.to_string().contains(STALE_PRICE) => {
                // this specific error message means that pool position
                // cannot be calculated - return None
                Ok(None)
            }
            Err(e) if e.to_string().contains(BLACK_SCHOLES) => {
                // this specific error message means that pool position
                // cannot be calculated - return None
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_value_of_lp_token(
        &self,
        block_number: i64,
        pool: FieldElement,
    ) -> Result<
        Option<FieldElement>,
        starknet::providers::ProviderError<SequencerGatewayProviderError>,
    > {
        match self
            .amm_call(
                block_number,
                vec![
                    pool,
                    // 10**18 as uint256
                    FieldElement::from_str(TEN_POW_18).unwrap(),
                    FieldElement::from_str("0").unwrap(),
                ],
                FunctionDescriptor {
                    name: "get_underlying_for_lptokens",
                    selector: selector!("get_underlying_for_lptokens"),
                },
            )
            .await
        {
            Ok(v) => Ok(Some(v)),
            Err(e) if e.to_string().contains(OPTION_NEAR_MATURITY) => {
                // this specific error message means that LP token value
                // cannot be calculated - return None
                Ok(None)
            }
            Err(e) if e.to_string().contains(STALE_PRICE) => {
                // this specific error message means that LP token value
                // cannot be calculated - return None
                Ok(None)
            }
            Err(e) if e.to_string().contains(BLACK_SCHOLES) => {
                // this specific error message means that pool position
                // cannot be calculated - return None
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_locked_unlocked_total_capital_for_pool(
        &self,
        pool: FieldElement,
        block_number: i64,
    ) -> Result<
        (
            FieldElement,
            FieldElement,
            FieldElement,
            Option<FieldElement>,
            Option<FieldElement>,
            FieldElement,
        ),
        starknet::providers::ProviderError<SequencerGatewayProviderError>,
    > {
        match try_join!(
            self.get_pool_locked_capital(block_number, pool),
            self.get_unlocked_capital(block_number, pool),
            self.get_lpool_balance(block_number, pool),
            self.get_value_of_pool_position(block_number, pool),
            self.get_value_of_lp_token(block_number, pool),
        ) {
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
                println!("{:?}", e);
                Err(e)
            }
        }
    }

    pub async fn get_amm_state(&self, block: &DbBlock) -> Result<Vec<PoolState>, ()> {
        let pool_addresses: Vec<FieldElement> = get_pools(&self.network)
            .iter()
            .map(|p| FieldElement::from_str(&p.lp_address).unwrap())
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
                unlocked_cap: to_hex(unlocked_cap),
                locked_cap: to_hex(locked_cap),
                lp_balance: to_hex(lpool_balance),
                pool_position: match value_pool_position {
                    Some(v) => Some(to_hex(v)),
                    None => None,
                },
                lp_address: to_hex(pool_address),
                block_number: block.block_number,
                lp_token_value: match lp_token_value {
                    Some(v) => Some(to_hex(v)),
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
        let options = get_options(&self.network);
        let mut to_store: Vec<OptionVolatility> = vec![];

        let mut futures = vec![];
        for opt in options {
            // until maturity + two days, keep getting volatility and position
            if opt.maturity + TWO_DAYS_SECS > block.timestamp {
                futures.push(self.get_option_volatility(opt, block.block_number));

            // else set volatility to None
            } else {
                let option_volatility = OptionVolatility {
                    block_number: block.block_number,
                    option_address: opt.option_address,
                    volatility: None,
                    option_position: None,
                };
                to_store.push(option_volatility);
            }
        }

        // await all options
        let results = join_all(futures).await;

        for res in results {
            let (volatility_option, option_position_option, option_address) = res;

            let volatility = volatility_option.map(|v| to_hex(v));
            let option_position = option_position_option.map(|v| to_hex(v));

            to_store.push(OptionVolatility {
                block_number: block.block_number,
                option_address,
                volatility,
                option_position,
            });
        }

        Ok(to_store)
    }

    pub async fn get_option_volatility(
        &self,
        opt: IOption,
        block_number: i64,
    ) -> (Option<FieldElement>, Option<FieldElement>, String) {
        let lp_address = FieldElement::from_str(opt.lp_address.as_str()).unwrap();
        let maturity = FieldElement::from_str(format!("{:#x}", opt.maturity).as_str()).unwrap();
        let strike = FieldElement::from_str(opt.strike_price.as_str()).unwrap();
        let side = FieldElement::from(opt.option_side as u8);

        let volatility_future = Box::pin(self.provider.call_contract(
            CallFunction {
                contract_address: self.amm_address,
                entry_point_selector: selector!("get_pool_volatility_auto"),
                calldata: vec![lp_address, maturity, strike],
            },
            BlockId::Number(block_number as u64),
        ));

        let position_future = Box::pin(self.provider.call_contract(
            CallFunction {
                contract_address: self.amm_address,
                entry_point_selector: selector!("get_option_position"),
                calldata: vec![lp_address, side, maturity, strike],
            },
            BlockId::Number(block_number as u64),
        ));

        let mut results = join_all(vec![volatility_future, position_future]).await;

        let volatility_result = results.remove(0);
        let position_result = results.remove(0);

        let volatility: Option<FieldElement> = match volatility_result {
            Ok(v) => Some(v.result[0]),
            Err(_) => None,
        };
        let position: Option<FieldElement> = match position_result {
            Ok(v) => Some(v.result[0]),
            Err(_) => None,
        };

        (volatility, position, opt.option_address)
    }

    pub async fn get_block_by_id(&self, block_id: BlockId) -> Result<Block, ()> {
        match self.provider.get_block(block_id).await {
            Ok(v) => Ok(v),
            Err(e) => {
                println!("Failed getting block {:?}", e);
                Err(())
            }
        }
    }

    pub async fn get_latest_block(&self) -> Result<Block, ()> {
        if let Ok(block) = self.get_block_by_id(BlockId::Latest).await {
            return Ok(block);
        }
        Err(())
    }
}
