use std::fmt;

// Starknet addresses
const TESTNET_CALL_LP_ADDRESS: &str =
    "0x3b176f8e5b4c9227b660e49e97f2d9d1756f96e5878420ad4accd301dd0cc17";
const TESTNET_PUT_LP_ADDRESS: &str =
    "0x30fe5d12635ed696483a824eca301392b3f529e06133b42784750503a24972";
const TESTNET_CONTRACT_ADDRESS: &str =
    "0x42a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54";
const MAINNET_CALL_LP_ADDRESS: &str =
    "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024";
const MAINNET_PUT_LP_ADDRESS: &str =
    "0x18a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a";
const MAINNET_CONTRACT_ADDRESS: &str =
    "0x76dbabc4293db346b0a56b29b6ea9fe18e93742c73f12348c8747ecfc1050aa";

const HASHSTACK_ADDRESS: &str =
    "0x03dcf5c72ba60eb7b2fe151032769d49dd3df6b04fa3141dffd6e2aa162b7a6e";
const ZKLEND_ADDRESS: &str = "0x04c0a5193d58f74fbace4b74dcf65481e734ed1714121bdc571da345540efa05";
const ZETH_ADDRESS: &str = "0x01b5bd713e72fdc5d63ffd83762f81297f6175a5e0a4771cdadbc1dd5fe72cb1";
const ZUSDC_ADDRESS: &str = "0x047ad51726d891f972e74e4ad858a261b43869f7126ce7436ee0b2529a98f486";
const ZUSDT_ADDRESS: &str = "0x00811d8da5dc8a2206ea7fd0b28627c2d77280a515126e62baa4d78e22714c4a";
const ZDAI_ADDRESS: &str = "0x062fa7afe1ca2992f8d8015385a279f49fad36299754fb1e9866f4f052289376";
const ZWBTC_ADDRESS: &str = "0x02b9ea3acdb23da566cee8e8beae3125a1458e720dea68c4a9a7a2d8eb5bbb4a";

// StarkScan API
const MAINNET_STARKSCAN_API_BASE_URL: &str = "https://api.starkscan.co/api/v0/events";
const TESTNET_STARKSCAN_API_BASE_URL: &str = "https://api-testnet.starkscan.co/api/v0/events";

pub enum Protocol {
    CarmineOptions,
    Hashstack,
    ZkLend,
    ZETH,
    ZUSDC,
    ZUSDT,
    ZDAI,
    ZWBTC,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Protocol::CarmineOptions => write!(f, "CarmineOptions"),
            Protocol::Hashstack => write!(f, "Hashstack"),
            Protocol::ZkLend => write!(f, "ZkLend"),
            Protocol::ZETH => write!(f, "zETH"),
            Protocol::ZUSDC => write!(f, "zUSDC"),
            Protocol::ZUSDT => write!(f, "zUSDT"),
            Protocol::ZDAI => write!(f, "zDAI"),
            Protocol::ZWBTC => write!(f, "zWBTC"),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Network {
    Testnet,
    Mainnet,
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Network::Testnet => write!(f, "Testnet"),
            Network::Mainnet => write!(f, "Mainnet"),
        }
    }
}

pub fn call_lp_address(network: &Network) -> &'static str {
    match &network {
        Network::Mainnet => MAINNET_CALL_LP_ADDRESS,
        Network::Testnet => TESTNET_CALL_LP_ADDRESS,
    }
}

pub fn put_lp_address(network: &Network) -> &'static str {
    match &network {
        Network::Mainnet => MAINNET_PUT_LP_ADDRESS,
        Network::Testnet => TESTNET_PUT_LP_ADDRESS,
    }
}

pub fn amm_address(network: &Network) -> &'static str {
    match &network {
        Network::Mainnet => MAINNET_CONTRACT_ADDRESS,
        Network::Testnet => TESTNET_CONTRACT_ADDRESS,
    }
}

pub fn starkscan_base_url(network: &Network) -> &'static str {
    match &network {
        Network::Mainnet => MAINNET_STARKSCAN_API_BASE_URL,
        Network::Testnet => TESTNET_STARKSCAN_API_BASE_URL,
    }
}

pub fn protocol_address(network: &Network, protocol: &Protocol) -> &'static str {
    match protocol {
        Protocol::CarmineOptions => match network {
            Network::Mainnet => {
                "0x076dbabc4293db346b0a56b29b6ea9fe18e93742c73f12348c8747ecfc1050aa"
            }
            Network::Testnet => {
                "0x042a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54"
            }
        },
        Protocol::Hashstack => HASHSTACK_ADDRESS,
        Protocol::ZkLend => ZKLEND_ADDRESS,
        Protocol::ZETH => ZETH_ADDRESS,
        Protocol::ZUSDC => ZUSDC_ADDRESS,
        Protocol::ZUSDT => ZUSDT_ADDRESS,
        Protocol::ZDAI => ZDAI_ADDRESS,
        Protocol::ZWBTC => ZWBTC_ADDRESS,
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn addresses_mainnet() {
        assert_eq!(
            call_lp_address(&Network::Mainnet),
            "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024"
        );
        assert_eq!(
            put_lp_address(&Network::Mainnet),
            "0x18a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a"
        );
        assert_eq!(
            amm_address(&Network::Mainnet),
            "0x76dbabc4293db346b0a56b29b6ea9fe18e93742c73f12348c8747ecfc1050aa"
        );
        assert_eq!(
            starkscan_base_url(&Network::Mainnet),
            "https://api.starkscan.co/api/v0/events"
        );
    }

    #[test]
    fn addresses_testnet() {
        assert_eq!(
            call_lp_address(&Network::Testnet),
            "0x3b176f8e5b4c9227b660e49e97f2d9d1756f96e5878420ad4accd301dd0cc17"
        );
        assert_eq!(
            put_lp_address(&Network::Testnet),
            "0x30fe5d12635ed696483a824eca301392b3f529e06133b42784750503a24972"
        );
        assert_eq!(
            amm_address(&Network::Testnet),
            "0x42a7d485171a01b8c38b6b37e0092f0f096e9d3f945c50c77799171916f5a54"
        );
        assert_eq!(
            starkscan_base_url(&Network::Testnet),
            "https://api-testnet.starkscan.co/api/v0/events"
        );
    }
}
