use std::fmt;

// Starknet addresses
const TESTNET_CALL_LP_ADDRESS: &str =
    "0x5b3bafb3afa300eeea9415d049b8694c6dc3e1e0e07d04d8f17766cb49745e";
const TESTNET_PUT_LP_ADDRESS: &str =
    "0x19c7d26452843d6458eac8027e8e0a4699e072c36280c6ef842297fd246d8d1";
const TESTNET_CONTRACT_ADDRESS: &str =
    "0x070eb12729e80d751e999557c9c1b0754a0c7933fbde0f310b99c8b6886e139e";
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
    NostraInterestModel,
    NostraETH,
    NostraETHCollateral,
    NostraETHInterest,
    NostraETHDebt,
    NostraETHInterestCollateral,
    NostraUSDC,
    NostraUSDCCollateral,
    NostraUSDCInterest,
    NostraUSDCDebt,
    NostraUSDCInterestCollateral,
    NostraUSDT,
    NostraUSDTCollateral,
    NostraUSDTInterest,
    NostraUSDTDebt,
    NostraUSDTInterestCollateral,
    NostraDAI,
    NostraDAICollateral,
    NostraDAIInterest,
    NostraDAIDebt,
    NostraDAIInterestCollateral,
    NostraWBTC,
    NostraWBTCCollateral,
    NostraWBTCInterest,
    NostraWBTCDebt,
    NostraWBTCInterestCollateral,
    Nostra2InterestModel,
    Nostra2ETH,
    Nostra2ETHCollateral,
    Nostra2ETHInterest,
    Nostra2ETHDebt,
    Nostra2ETHInterestCollateral,
    Nostra2USDC,
    Nostra2USDCCollateral,
    Nostra2USDCInterest,
    Nostra2USDCDebt,
    Nostra2USDCInterestCollateral,
    Nostra2USDT,
    Nostra2USDTCollateral,
    Nostra2USDTInterest,
    Nostra2USDTDebt,
    Nostra2USDTInterestCollateral,
    Nostra2DAI,
    Nostra2DAICollateral,
    Nostra2DAIInterest,
    Nostra2DAIDebt,
    Nostra2DAIInterestCollateral,
    Nostra2WBTC,
    Nostra2WBTCCollateral,
    Nostra2WBTCInterest,
    Nostra2WBTCDebt,
    Nostra2WBTCInterestCollateral,
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
            Protocol::NostraInterestModel => write!(f, "NostraInterestModel"),
            Protocol::NostraETH => write!(f, "NostraETH"),
            Protocol::NostraETHCollateral => write!(f, "NostraETHCollateral"),
            Protocol::NostraETHInterest => write!(f, "NostraETHInterest"),
            Protocol::NostraETHDebt => write!(f, "NostraETHDebt"),
            Protocol::NostraETHInterestCollateral => write!(f, "NostraETHInterestCollateral"),
            Protocol::NostraUSDC => write!(f, "NostraUSDC"),
            Protocol::NostraUSDCCollateral => write!(f, "NostraUSDCCollateral"),
            Protocol::NostraUSDCInterest => write!(f, "NostraUSDCInterest"),
            Protocol::NostraUSDCDebt => write!(f, "NostraUSDCDebt"),
            Protocol::NostraUSDCInterestCollateral => write!(f, "NostraUSDCInterestCollateral"),
            Protocol::NostraUSDT => write!(f, "NostraUSDT"),
            Protocol::NostraUSDTCollateral => write!(f, "NostraUSDTCollateral"),
            Protocol::NostraUSDTInterest => write!(f, "NostraUSDTInterest"),
            Protocol::NostraUSDTDebt => write!(f, "NostraUSDTDebt"),
            Protocol::NostraUSDTInterestCollateral => write!(f, "NostraUSDTInterestCollateral"),
            Protocol::NostraDAI => write!(f, "NostraDAI"),
            Protocol::NostraDAICollateral => write!(f, "NostraDAICollateral"),
            Protocol::NostraDAIInterest => write!(f, "NostraDAIInterest"),
            Protocol::NostraDAIDebt => write!(f, "NostraDAIDebt"),
            Protocol::NostraDAIInterestCollateral => write!(f, "NostraDAIInterestCollateral"),
            Protocol::NostraWBTC => write!(f, "NostraWBTC"),
            Protocol::NostraWBTCCollateral => write!(f, "NostraWBTCCollateral"),
            Protocol::NostraWBTCInterest => write!(f, "NostraWBTCInterest"),
            Protocol::NostraWBTCDebt => write!(f, "NostraWBTCDebt"),
            Protocol::NostraWBTCInterestCollateral => write!(f, "NostraWBTCInterestCollateral"),
            Protocol::Nostra2InterestModel => write!(f, "Nostra2InterestModel"),
            Protocol::Nostra2ETH => write!(f, "Nostra2ETH"),
            Protocol::Nostra2ETHCollateral => write!(f, "Nostra2ETHCollateral"),
            Protocol::Nostra2ETHInterest => write!(f, "Nostra2ETHInterest"),
            Protocol::Nostra2ETHDebt => write!(f, "Nostra2ETHDebt"),
            Protocol::Nostra2ETHInterestCollateral => write!(f, "Nostra2ETHInterestCollateral"),
            Protocol::Nostra2USDC => write!(f, "Nostra2USDC"),
            Protocol::Nostra2USDCCollateral => write!(f, "Nostra2USDCCollateral"),
            Protocol::Nostra2USDCInterest => write!(f, "Nostra2USDCInterest"),
            Protocol::Nostra2USDCDebt => write!(f, "Nostra2USDCDebt"),
            Protocol::Nostra2USDCInterestCollateral => write!(f, "Nostra2USDCInterestCollateral"),
            Protocol::Nostra2USDT => write!(f, "Nostra2USDT"),
            Protocol::Nostra2USDTCollateral => write!(f, "Nostra2USDTCollateral"),
            Protocol::Nostra2USDTInterest => write!(f, "Nostra2USDTInterest"),
            Protocol::Nostra2USDTDebt => write!(f, "Nostra2USDTDebt"),
            Protocol::Nostra2USDTInterestCollateral => write!(f, "Nostra2USDTInterestCollateral"),
            Protocol::Nostra2DAI => write!(f, "Nostra2DAI"),
            Protocol::Nostra2DAICollateral => write!(f, "Nostra2DAICollateral"),
            Protocol::Nostra2DAIInterest => write!(f, "Nostra2DAIInterest"),
            Protocol::Nostra2DAIDebt => write!(f, "Nostra2DAIDebt"),
            Protocol::Nostra2DAIInterestCollateral => write!(f, "Nostra2DAIInterestCollateral"),
            Protocol::Nostra2WBTC => write!(f, "Nostra2WBTC"),
            Protocol::Nostra2WBTCCollateral => write!(f, "Nostra2WBTCCollateral"),
            Protocol::Nostra2WBTCInterest => write!(f, "Nostra2WBTCInterest"),
            Protocol::Nostra2WBTCDebt => write!(f, "Nostra2WBTCDebt"),
            Protocol::Nostra2WBTCInterestCollateral => write!(f, "Nostra2WBTCInterestCollateral"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
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
            Network::Testnet => TESTNET_CONTRACT_ADDRESS,
        },
        Protocol::Hashstack => HASHSTACK_ADDRESS,
        Protocol::ZkLend => ZKLEND_ADDRESS,
        Protocol::ZETH => ZETH_ADDRESS,
        Protocol::ZUSDC => ZUSDC_ADDRESS,
        Protocol::ZUSDT => ZUSDT_ADDRESS,
        Protocol::ZDAI => ZDAI_ADDRESS,
        Protocol::ZWBTC => ZWBTC_ADDRESS,
        Protocol::NostraInterestModel => {
            "0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892"
        }
        Protocol::NostraETH => "0x04f89253e37ca0ab7190b2e9565808f105585c9cacca6b2fa6145553fa061a41",
        Protocol::NostraETHCollateral => {
            "0x0553cea5d1dc0e0157ffcd36a51a0ced717efdadd5ef1b4644352bb45bd35453"
        }
        Protocol::NostraETHInterest => {
            "0x002f8deaebb9da2cb53771b9e2c6d67265d11a4e745ebd74a726b8859c9337b9"
        }
        Protocol::NostraETHDebt => {
            "0x040b091cb020d91f4a4b34396946b4d4e2a450dbd9410432ebdbfe10e55ee5e5"
        }
        Protocol::NostraETHInterestCollateral => {
            "0x070f8a4fcd75190661ca09a7300b7c93fab93971b67ea712c664d7948a8a54c6"
        }
        Protocol::NostraUSDC => {
            "0x05327df4c669cb9be5c1e2cf79e121edef43c1416fac884559cd94fcb7e6e232"
        }
        Protocol::NostraUSDCCollateral => {
            "0x047e794d7c49c49fd2104a724cfa69a92c5a4b50a5753163802617394e973833"
        }
        Protocol::NostraUSDCInterest => {
            "0x06af9a313434c0987f5952277f1ac8c61dc4d50b8b009539891ed8aaee5d041d"
        }
        Protocol::NostraUSDCDebt => {
            "0x03b6058a9f6029b519bc72b2cc31bcb93ca704d0ab79fec2ae5d43f79ac07f7a"
        }
        Protocol::NostraUSDCInterestCollateral => {
            "0x029959a546dda754dc823a7b8aa65862c5825faeaaf7938741d8ca6bfdc69e4e"
        }
        Protocol::NostraUSDT => {
            "0x040375d0720245bc0d123aa35dc1c93d14a78f64456eff75f63757d99a0e6a83"
        }
        Protocol::NostraUSDTCollateral => {
            "0x003cd2066f3c8b4677741b39db13acebba843bbbaa73d657412102ab4fd98601"
        }
        Protocol::NostraUSDTInterest => {
            "0x06404c8e886fea27590710bb0e0e8c7a3e7d74afccc60663beb82707495f8609"
        }
        Protocol::NostraUSDTDebt => {
            "0x065c6c7119b738247583286021ea05acc6417aa86d391dcdda21843c1fc6e9c6"
        }
        Protocol::NostraUSDTInterestCollateral => {
            "0x055ba2baf189b98c59f6951a584a3a7d7d6ff2c4ef88639794e739557e1876f0"
        }
        Protocol::NostraDAI => "0x02ea39ba7a05f0c936b7468d8bc8d0e1f2116916064e7e163e7c1044d95bd135",
        Protocol::NostraDAICollateral => {
            "0x04403e420521e7a4ca0dc5192af81ca0bb36de343564a9495e11c8d9ba6e9d17"
        }
        Protocol::NostraDAIInterest => {
            "0x00b9b1a4373de5b1458e598df53195ea3204aa926f46198b50b32ed843ce508b"
        }
        Protocol::NostraDAIDebt => {
            "0x0362b4455f5f4cc108a5a1ab1fd2cc6c4f0c70597abb541a99cf2734435ec9cb"
        }
        Protocol::NostraDAIInterestCollateral => {
            "0x01ac55cabf2b79cf39b17ba0b43540a64205781c4b7850e881014aea6f89be58"
        }
        Protocol::NostraWBTC => {
            "0x07788bc687f203b6451f2a82e842b27f39c7cae697dace12edfb86c9b1c12f3d"
        }
        Protocol::NostraWBTCCollateral => {
            "0x06b59e2a746e141f90ec8b6e88e695265567ab3bdcf27059b4a15c89b0b7bd53"
        }
        Protocol::NostraWBTCInterest => {
            "0x0061d892cccf43daf73407194da9f0ea6dbece950bb24c50be2356444313a707"
        }
        Protocol::NostraWBTCDebt => {
            "0x075b0d87aca8dee25df35cdc39a82b406168fa23a76fc3f03abbfdc6620bb6d7"
        }
        Protocol::NostraWBTCInterestCollateral => {
            "0x00687b5d9e591844169bc6ad7d7256c4867a10cee6599625b9d78ea17a7caef9"
        }
        Protocol::Nostra2InterestModel => {
            "0x059a943ca214c10234b9a3b61c558ac20c005127d183b86a99a8f3c60a08b4ff"
        }
        Protocol::Nostra2ETH => {
            "0x07170f54dd61ae85377f75131359e3f4a12677589bb7ec5d61f362915a5c0982"
        }
        Protocol::Nostra2ETHCollateral => {
            "0x044debfe17e4d9a5a1e226dabaf286e72c9cc36abbe71c5b847e669da4503893"
        }
        Protocol::Nostra2ETHInterest => {
            "0x01fecadfe7cda2487c66291f2970a629be8eecdcb006ba4e71d1428c2b7605c7"
        }
        Protocol::Nostra2ETHDebt => {
            "0x00ba3037d968790ac486f70acaa9a1cab10cf5843bb85c986624b4d0e5a82e74"
        }
        Protocol::Nostra2ETHInterestCollateral => {
            "0x057146f6409deb4c9fa12866915dd952aa07c1eb2752e451d7f3b042086bdeb8"
        }
        Protocol::Nostra2USDC => {
            "0x06eda767a143da12f70947192cd13ee0ccc077829002412570a88cd6539c1d85"
        }
        Protocol::Nostra2USDCCollateral => {
            "0x05f296e1b9f4cf1ab452c218e72e02a8713cee98921dad2d3b5706235e128ee4"
        }
        Protocol::Nostra2USDCInterest => {
            "0x002fc2d4b41cc1f03d185e6681cbd40cced61915d4891517a042658d61cba3b1"
        }
        Protocol::Nostra2USDCDebt => {
            "0x063d69ae657bd2f40337c39bf35a870ac27ddf91e6623c2f52529db4c1619a51"
        }
        Protocol::Nostra2USDCInterestCollateral => {
            "0x05dcd26c25d9d8fd9fc860038dcb6e4d835e524eb8a85213a8cda5b7fff845f6"
        }
        Protocol::Nostra2USDT => {
            "0x06669cb476aa7e6a29c18b59b54f30b8bfcfbb8444f09e7bbb06c10895bf5d7b"
        }
        Protocol::Nostra2USDTCollateral => {
            "0x057717edc5b1e56743e8153be626729eb0690b882466ef0cbedc8a28bb4973b1"
        }
        Protocol::Nostra2USDTInterest => {
            "0x0360f9786a6595137f84f2d6931aaec09ceec476a94a98dcad2bb092c6c06701"
        }
        Protocol::Nostra2USDTDebt => {
            "0x024e9b0d6bc79e111e6872bb1ada2a874c25712cf08dfc5bcf0de008a7cca55f"
        }
        Protocol::Nostra2USDTInterestCollateral => {
            "0x0453c4c996f1047d9370f824d68145bd5e7ce12d00437140ad02181e1d11dc83"
        }
        Protocol::Nostra2DAI => {
            "0x02b5fd690bb9b126e3517f7abfb9db038e6a69a068303d06cf500c49c1388e20"
        }
        Protocol::Nostra2DAICollateral => {
            "0x005c4676bcb21454659479b3cd0129884d914df9c9b922c1c649696d2e058d70"
        }
        Protocol::Nostra2DAIInterest => {
            "0x022ccca3a16c9ef0df7d56cbdccd8c4a6f98356dfd11abc61a112483b242db90"
        }
        Protocol::Nostra2DAIDebt => {
            "0x066037c083c33330a8460a65e4748ceec275bbf5f28aa71b686cbc0010e12597"
        }
        Protocol::Nostra2DAIInterestCollateral => {
            "0x04f18ffc850cdfa223a530d7246d3c6fc12a5969e0aa5d4a88f470f5fe6c46e9"
        }
        Protocol::Nostra2WBTC => {
            "0x073132577e25b06937c64787089600886ede6202d085e6340242a5a32902e23e"
        }
        Protocol::Nostra2WBTCCollateral => {
            "0x036b68238f3a90639d062669fdec08c4d0bdd09826b1b6d24ef49de6d8141eaa"
        }
        Protocol::Nostra2WBTCInterest => {
            "0x0735d0f09a4e8bf8a17005fa35061b5957dcaa56889fc75df9e94530ff6991ea"
        }
        Protocol::Nostra2WBTCDebt => {
            "0x0491480f21299223b9ce770f23a2c383437f9fbf57abc2ac952e9af8cdb12c97"
        }
        Protocol::Nostra2WBTCInterestCollateral => {
            "0x05b7d301fa769274f20e89222169c0fad4d846c366440afc160aafadd6f88f0c"
        }
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
