use std::env;

// Starknet addresses
const TESTNET_CALL_LP_ADDRESS: &str =
    "0x03b176f8e5b4c9227b660e49e97f2d9d1756f96e5878420ad4accd301dd0cc17";
const TESTNET_PUT_LP_ADDRESS: &str =
    "0x0030fe5d12635ed696483a824eca301392b3f529e06133b42784750503a24972";
const TESTNET_CONTRACT_ADDRESS: &str =
    "0x042a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54";
const MAINNET_CALL_LP_ADDRESS: &str = "mainnet_call_lp_address";
const MAINNET_PUT_LP_ADDRESS: &str = "mainnet_put_lp_address";
const MAINNET_CONTRACT_ADDRESS: &str = "mainnet_amm_contract_address";

// StarkScan API
const MAINNET_STARKSCAN_API_BASE_URL: &str = "https://api.starkscan.co/api/v0/events";
const TESTNET_STARKSCAN_API_BASE_URL: &str = "https://api-testnet.starkscan.co/api/v0/events";

fn address_per_network(testnet: &'static str, mainnet: &'static str) -> &'static str {
    match env::var("NETWORK") {
        Ok(network) if network == String::from("mainnet") => mainnet,
        _ => testnet,
    }
}

pub fn call_lp_address() -> &'static str {
    address_per_network(TESTNET_CALL_LP_ADDRESS, MAINNET_CALL_LP_ADDRESS)
}

pub fn put_lp_address() -> &'static str {
    address_per_network(TESTNET_PUT_LP_ADDRESS, MAINNET_PUT_LP_ADDRESS)
}

pub fn amm_address() -> &'static str {
    address_per_network(TESTNET_CONTRACT_ADDRESS, MAINNET_CONTRACT_ADDRESS)
}

pub fn starkscan_base_url() -> &'static str {
    address_per_network(
        TESTNET_STARKSCAN_API_BASE_URL,
        MAINNET_STARKSCAN_API_BASE_URL,
    )
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn addresses_mainnet() {
        env::set_var("NETWORK", "mainnet");
        assert_eq!(call_lp_address(), "mainnet_call_lp_address");
        assert_eq!(put_lp_address(), "mainnet_put_lp_address");
        assert_eq!(amm_address(), "mainnet_amm_contract_address");
        assert_eq!(
            starkscan_base_url(),
            "https://api.starkscan.co/api/v0/events"
        );
    }

    #[test]
    fn addresses_testnet() {
        env::set_var("NETWORK", "testnet");
        assert_eq!(
            call_lp_address(),
            "0x03b176f8e5b4c9227b660e49e97f2d9d1756f96e5878420ad4accd301dd0cc17"
        );
        assert_eq!(
            put_lp_address(),
            "0x0030fe5d12635ed696483a824eca301392b3f529e06133b42784750503a24972"
        );
        assert_eq!(
            amm_address(),
            "0x042a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54"
        );
        assert_eq!(
            starkscan_base_url(),
            "https://api-testnet.starkscan.co/api/v0/events"
        );
    }
}
