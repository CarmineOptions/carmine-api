use crate::network::Network;
use std::fmt;

#[derive(Debug)]
pub struct Token {
    pub address: &'static str,
    pub symbol: &'static str,
    pub decimals: u8,
}

#[derive(Debug)]
pub struct Pool {
    pub address: &'static str,
    pub network: Network,
    pub type_: Type,
    pub base: Token,
    pub quote: Token,
    pub id: &'static str,
}

#[derive(Debug)]
pub enum Type {
    Call = 0,
    Put = 1,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Call => write!(f, "Call"),
            Type::Put => write!(f, "Put"),
        }
    }
}

const TESTNET_ETH: Token = Token {
    address: "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    decimals: 18,
    symbol: "ETH",
};

const TESTNET_USDC: Token = Token {
    address: "0x5a643907b9a4bc6a55e9069c4fd5fd1f5c79a22470690f75556c4736e34426",
    decimals: 6,
    symbol: "USDC",
};

const TESTNET_BTC: Token = Token {
    address: "0x12d537dc323c439dc65c976fad242d5610d27cfb5f31689a0a319b8be7f3d56",
    decimals: 8,
    symbol: "BTC",
};

const MAINNET_ETH: Token = Token {
    address: "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    decimals: 18,
    symbol: "ETH",
};

const MAINNET_USDC: Token = Token {
    address: "0x53c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
    decimals: 6,
    symbol: "USDC",
};

#[allow(dead_code)]
// BTC not yet implemented for the Mainnet
const MAINNET_BTC: Token = Token {
    address: "0x3fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac",
    decimals: 8,
    symbol: "BTC",
};

const TESTNET_ETH_USDC_CALL: Pool = Pool {
    address: "0x73e39528d223b3bb7a27400516120f634564f14d45cddf4ba04834d083f2968",
    network: Network::Testnet,
    type_: Type::Call,
    base: TESTNET_ETH,
    quote: TESTNET_USDC,
    id: "eth-usdc-call",
};

const TESTNET_ETH_USDC_PUT: Pool = Pool {
    address: "0x519ddd4a869bc75fdfccaf40af1c8aa42ea34b703391d248b2098513ed2e98e",
    network: Network::Testnet,
    type_: Type::Put,
    base: TESTNET_ETH,
    quote: TESTNET_USDC,
    id: "eth-usdc-put",
};

const TESTNET_BTC_USDC_CALL: Pool = Pool {
    address: "0x4a6016451ec67270b2c2e6b3431343053891ef28ee01fc613923e7d3c61ee1",
    network: Network::Testnet,
    type_: Type::Call,
    base: TESTNET_BTC,
    quote: TESTNET_USDC,
    id: "btc-usdc-call",
};

const TESTNET_BTC_USDC_PUT: Pool = Pool {
    address: "0x7a82787b8fc4a90dd5e3e573d95e32606c972747bce6600711dcfea9fb8c868",
    network: Network::Testnet,
    type_: Type::Put,
    base: TESTNET_BTC,
    quote: TESTNET_USDC,
    id: "btc-usdc-put",
};

const MAINNET_ETH_USDC_CALL: Pool = Pool {
    address: "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024",
    network: Network::Mainnet,
    type_: Type::Call,
    base: MAINNET_ETH,
    quote: MAINNET_USDC,
    id: "eth-usdc-call",
};

const MAINNET_ETH_USDC_PUT: Pool = Pool {
    address: "0x18a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a",
    network: Network::Mainnet,
    type_: Type::Put,
    base: MAINNET_ETH,
    quote: MAINNET_USDC,
    id: "eth-usdc-put",
};

pub fn get_all_pools(network: &Network) -> Vec<Pool> {
    match network {
        Network::Mainnet => vec![MAINNET_ETH_USDC_CALL, MAINNET_ETH_USDC_PUT],
        Network::Testnet => vec![
            TESTNET_ETH_USDC_CALL,
            TESTNET_ETH_USDC_PUT,
            TESTNET_BTC_USDC_CALL,
            TESTNET_BTC_USDC_PUT,
        ],
    }
}

pub fn get_all_pool_addresses(network: &Network) -> Vec<&'static str> {
    get_all_pools(network)
        .iter()
        .map(|pool| pool.address)
        .collect()
}
