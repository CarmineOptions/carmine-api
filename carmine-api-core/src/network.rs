use std::fmt;

// Starknet addresses
const TESTNET_CALL_LP_ADDRESS: &str =
    "0x5822f9be49b7c92402b16badc5fa30dd01689463db1081b59ec6e989c10cae5";
const TESTNET_PUT_LP_ADDRESS: &str =
    "0x5eb2ddbe4dc9cd04018e1614756f4321cce211d3a8690fd9688ae4a314e9d9";
const TESTNET_CONTRACT_ADDRESS: &str =
    "0x282530d787351ad7a90fdf0ecd52c6fa2ba57452cc08ea0309d1141c4356387";
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

#[derive(Debug)]
pub enum Protocol {
    CarmineOptions,
    Hashstack,
    Hashstack2,
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
    HashstackBTCRToken,
    HashstackBTCDToken,
    HashstackETHRToken,
    HashstackETHDToken,
    HashstackUSDTRToken,
    HashstackUSDTDToken,
    HashstackUSDCRToken,
    HashstackUSDCDToken,
    HashstackDAIRToken,
    HashstackDAIDToken,
    HashstackStaking,
    HashstackDiamond,
    HashstackL3Diamond,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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
        Protocol::Hashstack2 => {
            "0x01ef7f9f8bf01678dc6d27e2c26fb7e8eac3812a24752e6a1d6a49d153bec9f3"
        }
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
            "0x0514bd7ee8c97d4286bd481c54aa0793e43edbfb7e1ab9784c4b30469dcf9313"
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
        Protocol::HashstackBTCRToken => {
            "0x01320a9910e78afc18be65e4080b51ecc0ee5c0a8b6cc7ef4e685e02b50e57ef"
        }
        Protocol::HashstackBTCDToken => {
            "0x02614c784267d2026042ab98588f90efbffaade8982567e93530db4ed41201cf"
        }
        Protocol::HashstackETHRToken => {
            "0x00436d8d078de345c11493bd91512eae60cd2713e05bcaa0bb9f0cba90358c6e"
        }
        Protocol::HashstackETHDToken => {
            "0x01ef7f9f8bf01678dc6d27e2c26fb7e8eac3812a24752e6a1d6a49d153bec9f3"
        }
        Protocol::HashstackUSDTRToken => {
            "0x05fa6cc6185eab4b0264a4134e2d4e74be11205351c7c91196cb27d5d97f8d21"
        }
        Protocol::HashstackUSDTDToken => {
            "0x012b8185e237dd0340340faeb3351dbe53f8a42f5a9bf974ddf90ced56e301c7"
        }
        Protocol::HashstackUSDCRToken => {
            "0x03bcecd40212e9b91d92bbe25bb3643ad93f0d230d93237c675f46fac5187e8c"
        }
        Protocol::HashstackUSDCDToken => {
            "0x021d8d8519f5464ec63c6b9a80a5229c5ddeed57ecded4c8a9dfc34e31b49990"
        }
        Protocol::HashstackDAIRToken => {
            "0x019c981ec23aa9cbac1cc1eb7f92cf09ea2816db9cbd932e251c86a2e8fb725f"
        }
        Protocol::HashstackDAIDToken => {
            "0x07eeed99c095f83716e465e2c52a3ec8f47b323041ddc4f97778ac0393b7f358"
        }
        Protocol::HashstackStaking => {
            "0x005950cbbb7dbdb2303671515bb9e41ca0bf8937dc5ba929eebd276a3db3f854"
        }
        Protocol::HashstackDiamond => {
            "0x01b862c518939339b950d0d21a3d4cc8ead102d6270850ac8544636e558fab68"
        }
        Protocol::HashstackL3Diamond => {
            "0x05bc2d5f739fd82c176fc420b7acdbbf856d35597bdc575338664e84379245df"
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
