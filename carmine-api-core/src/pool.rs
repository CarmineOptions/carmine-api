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

const MAINNET_BTC: Token = Token {
    address: "0x3fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac",
    decimals: 8,
    symbol: "BTC",
};

const MAINNET_STRK: Token = Token {
    address: "0x4718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d",
    decimals: 18,
    symbol: "STRK",
};

const TESTNET_ETH_USDC_CALL: Pool = Pool {
    address: "0x5822f9be49b7c92402b16badc5fa30dd01689463db1081b59ec6e989c10cae5",
    network: Network::Testnet,
    type_: Type::Call,
    base: TESTNET_ETH,
    quote: TESTNET_USDC,
    id: "eth-usdc-call",
};

const TESTNET_ETH_USDC_PUT: Pool = Pool {
    address: "0x5eb2ddbe4dc9cd04018e1614756f4321cce211d3a8690fd9688ae4a314e9d9",
    network: Network::Testnet,
    type_: Type::Put,
    base: TESTNET_ETH,
    quote: TESTNET_USDC,
    id: "eth-usdc-put",
};

const TESTNET_BTC_USDC_CALL: Pool = Pool {
    address: "0x117cd5721ba22a6a758fbb06071a6a14137ba975f748828973e93167314dc01",
    network: Network::Testnet,
    type_: Type::Call,
    base: TESTNET_BTC,
    quote: TESTNET_USDC,
    id: "btc-usdc-call",
};

const TESTNET_BTC_USDC_PUT: Pool = Pool {
    address: "0x3423b6b6e166b2aa067a67e8d387e0bdee978c109b049921e8019b9c882dc86",
    network: Network::Testnet,
    type_: Type::Put,
    base: TESTNET_BTC,
    quote: TESTNET_USDC,
    id: "btc-usdc-put",
};

pub const MAINNET_ETH_USDC_CALL: Pool = Pool {
    address: "0x70cad6be2c3fc48c745e4a4b70ef578d9c79b46ffac4cd93ec7b61f951c7c5c",
    network: Network::Mainnet,
    type_: Type::Call,
    base: MAINNET_ETH,
    quote: MAINNET_USDC,
    id: "eth-usdc-call",
};

pub const MAINNET_ETH_USDC_PUT: Pool = Pool {
    address: "0x466e3a6731571cf5d74c5b0d9c508bfb71438de10f9a13269177b01d6f07159",
    network: Network::Mainnet,
    type_: Type::Put,
    base: MAINNET_ETH,
    quote: MAINNET_USDC,
    id: "eth-usdc-put",
};

pub const MAINNET_BTC_USDC_CALL: Pool = Pool {
    address: "0x35db72a814c9b30301f646a8fa8c192ff63a0dc82beb390a36e6e9eba55b6db",
    network: Network::Mainnet,
    type_: Type::Call,
    base: MAINNET_BTC,
    quote: MAINNET_USDC,
    id: "btc-usdc-call",
};

pub const MAINNET_BTC_USDC_PUT: Pool = Pool {
    address: "0x1bf27366077765c922f342c8de257591d1119ebbcbae7a6c4ff2f50ede4c54c",
    network: Network::Mainnet,
    type_: Type::Put,
    base: MAINNET_BTC,
    quote: MAINNET_USDC,
    id: "btc-usdc-put",
};

pub const MAINNET_ETH_STRK_CALL: Pool = Pool {
    address: "0x6df66db6a4b321869b3d1808fc702713b6cbb69541d583d4b38e7b1406c09aa",
    network: Network::Mainnet,
    type_: Type::Call,
    base: MAINNET_ETH,
    quote: MAINNET_STRK,
    id: "eth-strk-call",
};

pub const MAINNET_ETH_STRK_PUT: Pool = Pool {
    address: "0x4dcd9632353ed56e47be78f66a55a04e2c1303ebcb8ec7ea4c53f4fdf3834ec",
    network: Network::Mainnet,
    type_: Type::Put,
    base: MAINNET_ETH,
    quote: MAINNET_STRK,
    id: "eth-strk-put",
};

pub const MAINNET_STRK_USDC_CALL: Pool = Pool {
    address: "0x2b629088a1d30019ef18b893cebab236f84a365402fa0df2f51ec6a01506b1d",
    network: Network::Mainnet,
    type_: Type::Call,
    base: MAINNET_STRK,
    quote: MAINNET_USDC,
    id: "strk-usdc-call",
};

pub const MAINNET_STRK_USDC_PUT: Pool = Pool {
    address: "0x6ebf1d8bd43b9b4c5d90fb337c5c0647b406c6c0045da02e6675c43710a326f",
    network: Network::Mainnet,
    type_: Type::Put,
    base: MAINNET_STRK,
    quote: MAINNET_USDC,
    id: "strk-usdc-put",
};

pub const LEGACY_MAINNET_ETH_USDC_CALL: Pool = Pool {
    address: "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024",
    network: Network::Mainnet,
    type_: Type::Call,
    base: MAINNET_ETH,
    quote: MAINNET_USDC,
    id: "eth-usdc-call",
};

pub const LEGACY_MAINNET_ETH_USDC_PUT: Pool = Pool {
    address: "0x18a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a",
    network: Network::Mainnet,
    type_: Type::Put,
    base: MAINNET_ETH,
    quote: MAINNET_USDC,
    id: "eth-usdc-put",
};

pub fn get_all_pools(network: &Network) -> Vec<Pool> {
    match network {
        Network::Mainnet => vec![
            MAINNET_ETH_USDC_CALL,
            MAINNET_ETH_USDC_PUT,
            MAINNET_BTC_USDC_CALL,
            MAINNET_BTC_USDC_PUT,
            MAINNET_ETH_STRK_CALL,
            MAINNET_ETH_STRK_PUT,
            MAINNET_STRK_USDC_CALL,
            MAINNET_STRK_USDC_PUT,
        ],
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
