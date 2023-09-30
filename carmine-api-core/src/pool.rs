use crate::network::Network;

pub struct Token {
    pub address: &'static str,
    pub symbol: &'static str,
    pub decimals: u8,
}

pub struct Pool {
    pub address: &'static str,
    pub network: Network,
    pub type_: u8,
    pub base: Token,
    pub quote: Token,
}

const TESTNET_ETH: Token = Token {
    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    decimals: 18,
    symbol: "ETH",
};

const TESTNET_USDC: Token = Token {
    address: "0x005a643907b9a4bc6a55e9069c4fd5fd1f5c79a22470690f75556c4736e34426",
    decimals: 6,
    symbol: "USDC",
};

const TESTNET_BTC: Token = Token {
    address: "0x012d537dc323c439dc65c976fad242d5610d27cfb5f31689a0a319b8be7f3d56",
    decimals: 8,
    symbol: "BTC",
};

const MAINNET_ETH: Token = Token {
    address: "0x049d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7",
    decimals: 18,
    symbol: "ETH",
};

const MAINNET_USDC: Token = Token {
    address: "0x053c91253bc9682c04929ca02ed00b3e423f6710d2ee7e0d5ebb06f3ecf368a8",
    decimals: 6,
    symbol: "USDC",
};

#[allow(dead_code)]
// BTC not yet implemented for the Mainnet
const MAINNET_BTC: Token = Token {
    address: "0x03fe2b97c1fd336e750087d68b9b867997fd64a2661ff3ca5a7c771641e8e7ac",
    decimals: 8,
    symbol: "BTC",
};

const TESTNET_ETH_USDC_CALL: Pool = Pool {
    address: "0x005b3bafb3afa300eeea9415d049b8694c6dc3e1e0e07d04d8f17766cb49745e",
    network: Network::Testnet,
    type_: 0, // call
    base: TESTNET_ETH,
    quote: TESTNET_USDC,
};

const TESTNET_ETH_USDC_PUT: Pool = Pool {
    address: "0x019c7d26452843d6458eac8027e8e0a4699e072c36280c6ef842297fd246d8d1",
    network: Network::Testnet,
    type_: 1, // put
    base: TESTNET_ETH,
    quote: TESTNET_USDC,
};

const TESTNET_BTC_USDC_CALL: Pool = Pool {
    address: "0x0617e5eb189df89432e8d939e48fd4772dbd88654abad280f85f5328b0a22436",
    network: Network::Testnet,
    type_: 0, // call
    base: TESTNET_BTC,
    quote: TESTNET_USDC,
};

const TESTNET_BTC_USDC_PUT: Pool = Pool {
    address: "0x00f75be882a3d7495be44114e20355cf428ec50e6926e83a2eefdd3daff4b73e",
    network: Network::Testnet,
    type_: 1, // put
    base: TESTNET_BTC,
    quote: TESTNET_USDC,
};

const MAINNET_ETH_USDC_CALL: Pool = Pool {
    address: "0x07aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024",
    network: Network::Mainnet,
    type_: 0, // call
    base: MAINNET_ETH,
    quote: MAINNET_USDC,
};

const MAINNET_ETH_USDC_PUT: Pool = Pool {
    address: "0x018a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a",
    network: Network::Mainnet,
    type_: 1, // put
    base: MAINNET_ETH,
    quote: MAINNET_USDC,
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
