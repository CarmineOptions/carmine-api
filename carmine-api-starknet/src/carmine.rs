use carmine_api_core::network::{amm_address, call_lp_address, put_lp_address, Network};
use carmine_api_core::types::{DbBlock, IOption, OptionVolatility, PoolState};
use carmine_api_db::{create_batch_of_options, get_option_with_address, get_options, get_pools};
use futures::future::join_all;
use futures::FutureExt;
use starknet::core::types::{Block, CallContractResult, CallFunction, FieldElement};
use starknet::macros::selector;
use starknet::providers::SequencerGatewayProvider;
use starknet::{self, core::types::BlockId, providers::Provider};
use std::str::FromStr;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tokio::try_join;

const OPTION_NEAR_MATURITY: &'static str =
    "Unable to calculate position value, please wait till option with maturity";
const STALE_PRICE: &'static str = "Received price which is over an hour old";
const BLACK_SCHOLES: &'static str = "Black scholes function failed when calculating";
const TWO_DAYS_SECS: i64 = 172800;

const MAX_RETRIES: usize = 5;

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

        let call = loop {
            match self
                .provider
                .call_contract(
                    CallFunction {
                        contract_address: self.amm_address,
                        entry_point_selector: entrypoint,
                        calldata: vec![self.call_lp_address],
                    },
                    BlockId::Latest,
                )
                .await
            {
                Ok(res) => break res,
                Err(e) => {
                    println!("Failed getting call options\n{:?}", e);
                    sleep(Duration::from_secs(10)).await;
                }
            };
        };

        let put = loop {
            match self
                .provider
                .call_contract(
                    CallFunction {
                        contract_address: self.amm_address,
                        entry_point_selector: entrypoint,
                        calldata: vec![self.put_lp_address],
                    },
                    BlockId::Latest,
                )
                .await
            {
                Ok(res) => break res,
                Err(e) => {
                    println!("Failed getting put options\n{:?}", e);
                    sleep(Duration::from_secs(10)).await;
                }
            };
        };

        let contract_results = vec![call, put];

        let mut fetched_data: Vec<String> = Vec::new();

        for result in contract_results {
            let mut formatted = format_call_contract_result(result);
            fetched_data.append(&mut formatted);
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

        let filtered_call: Vec<CallContractResult> = contract_results
            .into_iter()
            .filter_map(|result| match result {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect();

        // only one option can be found
        assert_eq!(
            filtered_call.len(),
            1,
            "Option Info - Unexpected number of results"
        );

        let call_res = &filtered_call[0];

        println!("{:?}", call_res);

        let data = &call_res.result;
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
        let lp_address = match option_side {
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
                println!("Failed getting options {:?}", provider_error);
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
    ) -> Result<CallContractResult, &str> {
        for retry in 0..=MAX_RETRIES {
            match self
                .provider
                .call_contract(
                    CallFunction {
                        contract_address: self.amm_address,
                        entry_point_selector: function_descriptor.selector,
                        calldata: calldata.to_vec(),
                    },
                    BlockId::Number(block_number as u64),
                )
                .await
            {
                Ok(call_result) => return Ok(call_result),
                Err(e) => {
                    if retry < MAX_RETRIES {
                        sleep(Duration::from_secs(1)).await; // Wait before retrying
                    } else {
                        println!("amm_call failed MAX_RETRIES: {:?}", e);
                        return Err("amm_call failed MAX_RETRIES");
                    }
                }
            }
        }
        unreachable!();
    }

    pub async fn get_pool_locked_capital(
        &self,
        block_number: i64,
        pool: FieldElement,
    ) -> Result<FieldElement, &str> {
        match self
            .amm_call(
                block_number,
                vec![pool],
                FunctionDescriptor {
                    name: "get_pool_locked_capital",
                    selector: selector!("get_pool_locked_capital"),
                },
            )
            .await
        {
            Ok(res) => {
                if res.result.is_empty() {
                    return Err("get_pool_locked_capital empty result");
                }
                Ok(res.result[0])
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_unlocked_capital(
        &self,
        block_number: i64,
        pool: FieldElement,
    ) -> Result<FieldElement, &str> {
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
            Ok(res) => {
                if res.result.is_empty() {
                    return Err("get_unlocked_capital empty result");
                }
                Ok(res.result[0])
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_lpool_balance(
        &self,
        block_number: i64,
        pool: FieldElement,
    ) -> Result<FieldElement, &str> {
        match self
            .amm_call(
                block_number,
                vec![pool],
                FunctionDescriptor {
                    name: "get_lpool_balance",
                    selector: selector!("get_lpool_balance"),
                },
            )
            .await
        {
            Ok(res) => {
                if res.result.is_empty() {
                    return Err("get_lpool_balance empty result");
                }
                Ok(res.result[0])
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_value_of_pool_position(
        &self,
        block_number: i64,
        pool: FieldElement,
    ) -> Result<Option<FieldElement>, &str> {
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
            Ok(res) => {
                if res.result.is_empty() {
                    return Ok(None);
                }
                Ok(Some(res.result[0]))
            }
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
    ) -> Result<Option<FieldElement>, &str> {
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
            Ok(res) => {
                if res.result.is_empty() {
                    return Ok(None);
                }
                Ok(Some(res.result[0]))
            }
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

    pub async fn option_exists(&self, option: &IOption, block_number: i64) -> bool {
        let lp_address = FieldElement::from_str(option.lp_address.as_str()).unwrap();
        let maturity = FieldElement::from_str(format!("{:#x}", option.maturity).as_str()).unwrap();
        let strike = FieldElement::from_str(option.strike_price.as_str()).unwrap();
        let side = FieldElement::from(option.option_side as u8);

        match self
            .amm_call(
                block_number,
                vec![lp_address, side, maturity, strike],
                FunctionDescriptor {
                    name: "get_option_token_address",
                    selector: selector!("get_option_token_address"),
                },
            )
            .await
        {
            Ok(res) => {
                let address = res.result[0];
                address.eq(&FieldElement::ZERO)
            }
            Err(e) => {
                println!("Failed option_exists: {:#?}", e);
                // assume it does not exist
                false
            }
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
        &str,
    > {
        let now = Instant::now();
        let res = try_join!(
            self.get_pool_locked_capital(block_number, pool),
            self.get_unlocked_capital(block_number, pool),
            self.get_lpool_balance(block_number, pool),
            self.get_value_of_pool_position(block_number, pool),
            self.get_value_of_lp_token(block_number, pool),
        );
        println!("Fetched AMM state in {:.2?}", now.elapsed());
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
        let now = Instant::now();

        let options = get_options(&self.network);
        let mut to_store: Vec<OptionVolatility> = vec![];

        let mut non_expired_options: Vec<IOption> = vec![];
        let mut recently_expired_options: Vec<IOption> = vec![];

        for opt in options {
            if opt.maturity > block.timestamp {
                if self.option_exists(&opt, block.block_number).await {
                    non_expired_options.push(opt);
                }
            } else if opt.maturity + TWO_DAYS_SECS > block.timestamp {
                if self.option_exists(&opt, block.block_number).await {
                    recently_expired_options.push(opt);
                }
            }
        }

        let mut futures = vec![];

        for opt in recently_expired_options {
            futures.push(self.get_option_volatility_and_position(opt, block.block_number, false));
        }

        for opt in non_expired_options {
            futures.push(self.get_option_volatility_and_position(opt, block.block_number, true));
        }

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

        println!("Options volatility fetched in {:.2?}", now.elapsed());
        Ok(to_store)
    }

    pub async fn option_volatility(
        &self,
        lp_address: FieldElement,
        maturity: FieldElement,
        strike: FieldElement,
        block_number: i64,
    ) -> Result<FieldElement, &str> {
        match self
            .amm_call(
                block_number,
                vec![lp_address, maturity, strike],
                FunctionDescriptor {
                    name: "get_pool_volatility_auto",
                    selector: selector!("get_pool_volatility_auto"),
                },
            )
            .await
        {
            Ok(res) => {
                if res.result.is_empty() {
                    return Err("option_volatility empty result");
                }
                Ok(res.result[0])
            }
            Err(e) => Err(e),
        }
    }

    pub async fn option_position(
        &self,
        lp_address: FieldElement,
        maturity: FieldElement,
        strike: FieldElement,
        side: FieldElement,
        block_number: i64,
    ) -> Result<FieldElement, &str> {
        match self
            .amm_call(
                block_number,
                vec![lp_address, side, maturity, strike],
                FunctionDescriptor {
                    name: "get_pool_volatility_auto",
                    selector: selector!("get_pool_volatility_auto"),
                },
            )
            .await
        {
            Ok(res) => {
                if res.result.is_empty() {
                    return Err("option_position empty result");
                }
                Ok(res.result[0])
            }
            Err(e) => Err(e),
        }
    }

    pub async fn get_option_volatility_and_position(
        &self,
        opt: IOption,
        block_number: i64,
        fetch_volatility: bool,
    ) -> (Option<FieldElement>, Option<FieldElement>, String) {
        let lp_address = FieldElement::from_str(opt.lp_address.as_str()).unwrap();
        let maturity = FieldElement::from_str(format!("{:#x}", opt.maturity).as_str()).unwrap();
        let strike = FieldElement::from_str(opt.strike_price.as_str()).unwrap();
        let side = FieldElement::from(opt.option_side as u8);

        // we need volatility on some options, position on all
        if fetch_volatility {
            let volatility_future =
                self.option_volatility(lp_address, maturity, strike, block_number);
            let position_future =
                self.option_position(lp_address, maturity, strike, side, block_number);

            match try_join!(volatility_future, position_future) {
                Ok((volatility, position)) => {
                    (Some(volatility), Some(position), opt.option_address)
                }
                Err(err) => {
                    println!(
                        "Failed getting option volatility and position for block {}: {}",
                        block_number, err
                    );
                    (None, None, opt.option_address)
                }
            }
        } else {
            let position = self
                .option_position(lp_address, maturity, strike, side, block_number)
                .await;

            match position {
                Ok(position) => (None, Some(position), opt.option_address),
                Err(err) => {
                    println!(
                        "Failed getting option position for block {}: {}",
                        block_number, err
                    );
                    (None, None, opt.option_address)
                }
            }
        }
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
