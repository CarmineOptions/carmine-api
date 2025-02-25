use std::fmt;

use crate::pool::{
    MAINNET_BTC_USDC_CALL, MAINNET_BTC_USDC_PUT, MAINNET_ETH_STRK_CALL, MAINNET_ETH_STRK_PUT,
    MAINNET_ETH_USDC_CALL, MAINNET_ETH_USDC_PUT, MAINNET_STRK_USDC_CALL, MAINNET_STRK_USDC_PUT,
};

pub const MAINNET_CONTRACT_ADDRESS: &str =
    "0x047472e6755afc57ada9550b6a3ac93129cc4b5f98f51c73e0644d129fd208d9";
pub const MAINNET_AUXILIARY_CONTRACT: &str =
    "0x03e174d3d7dce00ad5e15299593a28c3defc660c77220867c921611a3aef4149";
pub const LEGACY_AMM_CONTRACT_ADDRESS: &str =
    "0x076dbabc4293db346b0a56b29b6ea9fe18e93742c73f12348c8747ecfc1050aa";

pub const TESTNET_CONTRACT_ADDRESS: &str =
    "0x07587280e108db0681eb60190ed1f5bd737965177f6c13551ab2e50d6644d382";
pub const TESTNET_AUXILIARY_CONTRACT: &str =
    "0x05bece17c00e2335bed4d0c0eae0c609545c207df15e319ee3fc556e9a134464";
pub const NEW_AMM_GENESIS_BLOCK_NUMBER: i64 = 504056;
pub const NEW_AMM_GENESIS_TIMESTAMP: i64 = 1705078858;

// StarkScan API
const MAINNET_STARKSCAN_API_BASE_URL: &str = "https://api.starkscan.co/api/v0/events";
const TESTNET_STARKSCAN_API_BASE_URL: &str = "https://api-testnet.starkscan.co/api/v0/events";

#[derive(Debug)]
pub enum Protocol {
    CarmineOptions,
    CarmineGovernance,
    LegacyCarminePoolEthUsdcCall,
    LegacyCarminePoolEthUsdcPut,
    LegacyCarmineOptions,
    CarminePoolEthUsdcCall,
    CarminePoolEthUsdcPut,
    CarminePoolBtcUsdcCall,
    CarminePoolBtcUsdcPut,
    CarminePoolEthStrkCall,
    CarminePoolEthStrkPut,
    CarminePoolStrkUsdcCall,
    CarminePoolStrkUsdcPut,
    Pail,
    ZkLendMarket,
    ZkLendETH,
    ZkLendUSDC,
    ZkLendUSDT,
    ZkLendWBTC,
    ZkLendDAI,
    NostraAlphaInterestModel,
    NostraAlphaETH,
    NostraAlphaETHCollateral,
    NostraAlphaETHInterest,
    NostraAlphaETHDebt,
    NostraAlphaETHInterestCollateral,
    NostraAlphaUSDC,
    NostraAlphaUSDCCollateral,
    NostraAlphaUSDCInterest,
    NostraAlphaUSDCDebt,
    NostraAlphaUSDCInterestCollateral,
    NostraAlphaUSDT,
    NostraAlphaUSDTCollateral,
    NostraAlphaUSDTInterest,
    NostraAlphaUSDTDebt,
    NostraAlphaUSDTInterestCollateral,
    NostraAlphaDAI,
    NostraAlphaDAICollateral,
    NostraAlphaDAIInterest,
    NostraAlphaDAIDebt,
    NostraAlphaDAIInterestCollateral,
    NostraAlphaWBTC,
    NostraAlphaWBTCCollateral,
    NostraAlphaWBTCInterest,
    NostraAlphaWBTCDebt,
    NostraAlphaWBTCInterestCollateral,
    NostraMainnetInterestModel,
    NostraMainnetETH,
    NostraMainnetETHCollateral,
    NostraMainnetETHInterest,
    NostraMainnetETHDebt,
    NostraMainnetETHInterestCollateral,
    NostraMainnetUSDC,
    NostraMainnetUSDCCollateral,
    NostraMainnetUSDCInterest,
    NostraMainnetUSDCDebt,
    NostraMainnetUSDCInterestCollateral,
    NostraMainnetUSDT,
    NostraMainnetUSDTCollateral,
    NostraMainnetUSDTInterest,
    NostraMainnetUSDTDebt,
    NostraMainnetUSDTInterestCollateral,
    NostraMainnetDAI,
    NostraMainnetDAICollateral,
    NostraMainnetDAIInterest,
    NostraMainnetDAIDebt,
    NostraMainnetDAIInterestCollateral,
    NostraMainnetWBTC,
    NostraMainnetWBTCCollateral,
    NostraMainnetWBTCInterest,
    NostraMainnetWBTCDebt,
    NostraMainnetWBTCInterestCollateral,
    NostraMainnetWSTETH,
    NostraMainnetWSTETHCollateral,
    NostraMainnetWSTETHInterest,
    NostraMainnetWSTETHDebt,
    NostraMainnetWSTETHInterestCollateral,
    NostraMainnetLORDS,
    NostraMainnetLORDSCollateral,
    NostraMainnetLORDSInterest,
    NostraMainnetLORDSDebt,
    NostraMainnetLORDSInterestCollateral,
    NostraMainnetSTRK,
    NostraMainnetSTRKCollateral,
    NostraMainnetSTRKInterest,
    NostraMainnetSTRKDebt,
    NostraMainnetSTRKInterestCollateral,
    NostraMainnetNSTSTRK,
    NostraMainnetNSTSTRKCollateral,
    NostraMainnetNSTSTRKInterest,
    NostraMainnetNSTSTRKDebt,
    NostraMainnetNSTSTRKInterestCollateral,
    NostraMainnetUNO,
    NostraMainnetUNOCollateral,
    NostraMainnetUNOInterest,
    NostraMainnetUNODebt,
    NostraMainnetUNOInterestCollateral,
    NostraMainnetNSTR,
    NostraMainnetNSTRCollateral,
    NostraMainnetNSTRInterest,
    NostraMainnetNSTRDebt,
    NostraMainnetNSTRInterestCollateral,
    NostraMainnetDAIV2,
    NostraMainnetDAIV2Interest,
    NostraMainnetDAIV2Debt,
    Hashstack,
    Hashstack2,
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
            Network::Mainnet => MAINNET_CONTRACT_ADDRESS,
            Network::Testnet => TESTNET_CONTRACT_ADDRESS,
        },
        Protocol::CarmineGovernance => {
            "0x001405ab78ab6ec90fba09e6116f373cda53b0ba557789a4578d8c1ec374ba0f"
        }
        Protocol::Pail => "0x038290ac85a923dd7b38c23cc1ec5b15853b76e2c3c02d367043685caecd2fc2",
        Protocol::LegacyCarminePoolEthUsdcCall => {
            "0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024"
        }
        Protocol::LegacyCarminePoolEthUsdcPut => {
            "0x18a6abca394bd5f822cfa5f88783c01b13e593d1603e7b41b00d31d2ea4827a"
        }
        Protocol::LegacyCarmineOptions => LEGACY_AMM_CONTRACT_ADDRESS,
        Protocol::CarminePoolEthUsdcCall => MAINNET_ETH_USDC_CALL.address,
        Protocol::CarminePoolEthUsdcPut => MAINNET_ETH_USDC_PUT.address,
        Protocol::CarminePoolBtcUsdcCall => MAINNET_BTC_USDC_CALL.address,
        Protocol::CarminePoolBtcUsdcPut => MAINNET_BTC_USDC_PUT.address,
        Protocol::CarminePoolEthStrkCall => MAINNET_ETH_STRK_CALL.address,
        Protocol::CarminePoolEthStrkPut => MAINNET_ETH_STRK_PUT.address,
        Protocol::CarminePoolStrkUsdcCall => MAINNET_STRK_USDC_CALL.address,
        Protocol::CarminePoolStrkUsdcPut => MAINNET_STRK_USDC_PUT.address,
        Protocol::ZkLendMarket => {
            "0x04c0a5193d58f74fbace4b74dcf65481e734ed1714121bdc571da345540efa05"
        }
        Protocol::ZkLendETH => "0x01b5bd713e72fdc5d63ffd83762f81297f6175a5e0a4771cdadbc1dd5fe72cb1",
        Protocol::ZkLendUSDC => {
            "0x047ad51726d891f972e74e4ad858a261b43869f7126ce7436ee0b2529a98f486"
        }
        Protocol::ZkLendUSDT => {
            "0x00811d8da5dc8a2206ea7fd0b28627c2d77280a515126e62baa4d78e22714c4a"
        }
        Protocol::ZkLendWBTC => {
            "0x02b9ea3acdb23da566cee8e8beae3125a1458e720dea68c4a9a7a2d8eb5bbb4a"
        }
        Protocol::ZkLendDAI => "0x062fa7afe1ca2992f8d8015385a279f49fad36299754fb1e9866f4f052289376",
        Protocol::NostraAlphaInterestModel => {
            "0x03d39f7248fb2bfb960275746470f7fb470317350ad8656249ec66067559e892"
        }
        Protocol::NostraAlphaETH => {
            "0x04f89253e37ca0ab7190b2e9565808f105585c9cacca6b2fa6145553fa061a41"
        }
        Protocol::NostraAlphaETHCollateral => {
            "0x0553cea5d1dc0e0157ffcd36a51a0ced717efdadd5ef1b4644352bb45bd35453"
        }
        Protocol::NostraAlphaETHInterest => {
            "0x002f8deaebb9da2cb53771b9e2c6d67265d11a4e745ebd74a726b8859c9337b9"
        }
        Protocol::NostraAlphaETHDebt => {
            "0x040b091cb020d91f4a4b34396946b4d4e2a450dbd9410432ebdbfe10e55ee5e5"
        }
        Protocol::NostraAlphaETHInterestCollateral => {
            "0x070f8a4fcd75190661ca09a7300b7c93fab93971b67ea712c664d7948a8a54c6"
        }
        Protocol::NostraAlphaUSDC => {
            "0x05327df4c669cb9be5c1e2cf79e121edef43c1416fac884559cd94fcb7e6e232"
        }
        Protocol::NostraAlphaUSDCCollateral => {
            "0x047e794d7c49c49fd2104a724cfa69a92c5a4b50a5753163802617394e973833"
        }
        Protocol::NostraAlphaUSDCInterest => {
            "0x06af9a313434c0987f5952277f1ac8c61dc4d50b8b009539891ed8aaee5d041d"
        }
        Protocol::NostraAlphaUSDCDebt => {
            "0x03b6058a9f6029b519bc72b2cc31bcb93ca704d0ab79fec2ae5d43f79ac07f7a"
        }
        Protocol::NostraAlphaUSDCInterestCollateral => {
            "0x029959a546dda754dc823a7b8aa65862c5825faeaaf7938741d8ca6bfdc69e4e"
        }
        Protocol::NostraAlphaUSDT => {
            "0x040375d0720245bc0d123aa35dc1c93d14a78f64456eff75f63757d99a0e6a83"
        }
        Protocol::NostraAlphaUSDTCollateral => {
            "0x003cd2066f3c8b4677741b39db13acebba843bbbaa73d657412102ab4fd98601"
        }
        Protocol::NostraAlphaUSDTInterest => {
            "0x06404c8e886fea27590710bb0e0e8c7a3e7d74afccc60663beb82707495f8609"
        }
        Protocol::NostraAlphaUSDTDebt => {
            "0x065c6c7119b738247583286021ea05acc6417aa86d391dcdda21843c1fc6e9c6"
        }
        Protocol::NostraAlphaUSDTInterestCollateral => {
            "0x055ba2baf189b98c59f6951a584a3a7d7d6ff2c4ef88639794e739557e1876f0"
        }
        Protocol::NostraAlphaDAI => {
            "0x02ea39ba7a05f0c936b7468d8bc8d0e1f2116916064e7e163e7c1044d95bd135"
        }
        Protocol::NostraAlphaDAICollateral => {
            "0x04403e420521e7a4ca0dc5192af81ca0bb36de343564a9495e11c8d9ba6e9d17"
        }
        Protocol::NostraAlphaDAIInterest => {
            "0x00b9b1a4373de5b1458e598df53195ea3204aa926f46198b50b32ed843ce508b"
        }
        Protocol::NostraAlphaDAIDebt => {
            "0x0362b4455f5f4cc108a5a1ab1fd2cc6c4f0c70597abb541a99cf2734435ec9cb"
        }
        Protocol::NostraAlphaDAIInterestCollateral => {
            "0x01ac55cabf2b79cf39b17ba0b43540a64205781c4b7850e881014aea6f89be58"
        }
        Protocol::NostraAlphaWBTC => {
            "0x07788bc687f203b6451f2a82e842b27f39c7cae697dace12edfb86c9b1c12f3d"
        }
        Protocol::NostraAlphaWBTCCollateral => {
            "0x06b59e2a746e141f90ec8b6e88e695265567ab3bdcf27059b4a15c89b0b7bd53"
        }
        Protocol::NostraAlphaWBTCInterest => {
            "0x0061d892cccf43daf73407194da9f0ea6dbece950bb24c50be2356444313a707"
        }
        Protocol::NostraAlphaWBTCDebt => {
            "0x075b0d87aca8dee25df35cdc39a82b406168fa23a76fc3f03abbfdc6620bb6d7"
        }
        Protocol::NostraAlphaWBTCInterestCollateral => {
            "0x00687b5d9e591844169bc6ad7d7256c4867a10cee6599625b9d78ea17a7caef9"
        }
        Protocol::NostraMainnetInterestModel => {
            "0x059a943ca214c10234b9a3b61c558ac20c005127d183b86a99a8f3c60a08b4ff"
        }
        Protocol::NostraMainnetETH => {
            "0x07170f54dd61ae85377f75131359e3f4a12677589bb7ec5d61f362915a5c0982"
        }
        Protocol::NostraMainnetETHCollateral => {
            "0x044debfe17e4d9a5a1e226dabaf286e72c9cc36abbe71c5b847e669da4503893"
        }
        Protocol::NostraMainnetETHInterest => {
            "0x01fecadfe7cda2487c66291f2970a629be8eecdcb006ba4e71d1428c2b7605c7"
        }
        Protocol::NostraMainnetETHDebt => {
            "0x00ba3037d968790ac486f70acaa9a1cab10cf5843bb85c986624b4d0e5a82e74"
        }
        Protocol::NostraMainnetETHInterestCollateral => {
            "0x057146f6409deb4c9fa12866915dd952aa07c1eb2752e451d7f3b042086bdeb8"
        }
        Protocol::NostraMainnetUSDC => {
            "0x06eda767a143da12f70947192cd13ee0ccc077829002412570a88cd6539c1d85"
        }
        Protocol::NostraMainnetUSDCCollateral => {
            "0x05f296e1b9f4cf1ab452c218e72e02a8713cee98921dad2d3b5706235e128ee4"
        }
        Protocol::NostraMainnetUSDCInterest => {
            "0x002fc2d4b41cc1f03d185e6681cbd40cced61915d4891517a042658d61cba3b1"
        }
        Protocol::NostraMainnetUSDCDebt => {
            "0x063d69ae657bd2f40337c39bf35a870ac27ddf91e6623c2f52529db4c1619a51"
        }
        Protocol::NostraMainnetUSDCInterestCollateral => {
            "0x05dcd26c25d9d8fd9fc860038dcb6e4d835e524eb8a85213a8cda5b7fff845f6"
        }
        Protocol::NostraMainnetUSDT => {
            "0x06669cb476aa7e6a29c18b59b54f30b8bfcfbb8444f09e7bbb06c10895bf5d7b"
        }
        Protocol::NostraMainnetUSDTCollateral => {
            "0x0514bd7ee8c97d4286bd481c54aa0793e43edbfb7e1ab9784c4b30469dcf9313"
        }
        Protocol::NostraMainnetUSDTInterest => {
            "0x0360f9786a6595137f84f2d6931aaec09ceec476a94a98dcad2bb092c6c06701"
        }
        Protocol::NostraMainnetUSDTDebt => {
            "0x024e9b0d6bc79e111e6872bb1ada2a874c25712cf08dfc5bcf0de008a7cca55f"
        }
        Protocol::NostraMainnetUSDTInterestCollateral => {
            "0x0453c4c996f1047d9370f824d68145bd5e7ce12d00437140ad02181e1d11dc83"
        }
        Protocol::NostraMainnetDAI => {
            "0x02b5fd690bb9b126e3517f7abfb9db038e6a69a068303d06cf500c49c1388e20"
        }
        Protocol::NostraMainnetDAICollateral => {
            "0x005c4676bcb21454659479b3cd0129884d914df9c9b922c1c649696d2e058d70"
        }
        Protocol::NostraMainnetDAIInterest => {
            "0x022ccca3a16c9ef0df7d56cbdccd8c4a6f98356dfd11abc61a112483b242db90"
        }
        Protocol::NostraMainnetDAIDebt => {
            "0x066037c083c33330a8460a65e4748ceec275bbf5f28aa71b686cbc0010e12597"
        }
        Protocol::NostraMainnetDAIInterestCollateral => {
            "0x04f18ffc850cdfa223a530d7246d3c6fc12a5969e0aa5d4a88f470f5fe6c46e9"
        }
        Protocol::NostraMainnetWBTC => {
            "0x073132577e25b06937c64787089600886ede6202d085e6340242a5a32902e23e"
        }
        Protocol::NostraMainnetWBTCCollateral => {
            "0x036b68238f3a90639d062669fdec08c4d0bdd09826b1b6d24ef49de6d8141eaa"
        }
        Protocol::NostraMainnetWBTCInterest => {
            "0x0735d0f09a4e8bf8a17005fa35061b5957dcaa56889fc75df9e94530ff6991ea"
        }
        Protocol::NostraMainnetWBTCDebt => {
            "0x0491480f21299223b9ce770f23a2c383437f9fbf57abc2ac952e9af8cdb12c97"
        }
        Protocol::NostraMainnetWBTCInterestCollateral => {
            "0x05b7d301fa769274f20e89222169c0fad4d846c366440afc160aafadd6f88f0c"
        }
        Protocol::NostraMainnetWSTETH => {
            "0x07e2c010c0b381f347926d5a203da0335ef17aefee75a89292ef2b0f94924864"
        }
        Protocol::NostraMainnetWSTETHCollateral => {
            "0x05eb6de9c7461b3270d029f00046c8a10d27d4f4a4c931a4ea9769c72ef4edbb"
        }
        Protocol::NostraMainnetWSTETHInterest => {
            "0x00ca44c79a77bcb186f8cdd1a0cd222cc258bebc3bec29a0a020ba20fdca40e9"
        }
        Protocol::NostraMainnetWSTETHDebt => {
            "0x0348cc417fc877a7868a66510e8e0d0f3f351f5e6b0886a86b652fcb30a3d1fb"
        }
        Protocol::NostraMainnetWSTETHInterestCollateral => {
            "0x009377fdde350e01e0397820ea83ed3b4f05df30bfb8cf8055d62cafa1b2106a"
        }
        Protocol::NostraMainnetLORDS => {
            "0x000d294e16a8d24c32eed65ea63757adde543d72bad4af3927f4c7c8969ff43d"
        }
        Protocol::NostraMainnetLORDSCollateral => {
            "0x02530a305dd3d92aad5cf97e373a3d07577f6c859337fb0444b9e851ee4a2dd4"
        }
        Protocol::NostraMainnetLORDSInterest => {
            "0x0507eb06dd372cb5885d3aaf18b980c41cd3cd4691cfd3a820339a6c0cec2674"
        }
        Protocol::NostraMainnetLORDSDebt => {
            "0x035778d24792bbebcf7651146896df5f787641af9e2a3db06480a637fbc9fff8"
        }
        Protocol::NostraMainnetLORDSInterestCollateral => {
            "0x0739760bce37f89b6c1e6b1198bb8dc7166b8cf21509032894f912c9d5de9cbd"
        }
        Protocol::NostraMainnetSTRK => {
            "0x07c535ddb7bf3d3cb7c033bd1a4c3aac02927a4832da795606c0f3dbbc6efd17"
        }
        Protocol::NostraMainnetSTRKCollateral => {
            "0x040f5a6b7a6d3c472c12ca31ae6250b462c6d35bbdae17bd52f6c6ca065e30cf"
        }
        Protocol::NostraMainnetSTRKInterest => {
            "0x026c5994c2462770bbf940552c5824fb0e0920e2a8a5ce1180042da1b3e489db"
        }
        Protocol::NostraMainnetSTRKDebt => {
            "0x001258eae3eae5002125bebf062d611a772e8aea3a1879b64a19f363ebd00947"
        }
        Protocol::NostraMainnetSTRKInterestCollateral => {
            "0x07c2e1e733f28daa23e78be3a4f6c724c0ab06af65f6a95b5e0545215f1abc1b"
        }
        Protocol::NostraMainnetNSTSTRK => {
            "0x04b11c750ae92c13fdcbe514f9c47ba6f8266c81014501baa8346d3b8ba55342"
        }
        Protocol::NostraMainnetNSTSTRKCollateral => {
            "0x0142af5b6c97f02cac9c91be1ea9895d855c5842825cb2180673796e54d73dc5"
        }
        Protocol::NostraMainnetNSTSTRKInterest => {
            "0x078a40c85846e3303bf7982289ca7def68297d4b609d5f588208ac553cff3a18"
        }
        Protocol::NostraMainnetNSTSTRKDebt => {
            "0x0292be6baee291a148006db984f200dbdb34b12fb2136c70bfe88649c12d934b"
        }
        Protocol::NostraMainnetNSTSTRKInterestCollateral => {
            "0x067a34ff63ec38d0ccb2817c6d3f01e8b0c4792c77845feb43571092dcf5ebb5"
        }
        Protocol::NostraMainnetUNO => {
            "0x06757ef9960c5bc711d1ba7f7a3bff44a45ba9e28f2ac0cc63ee957e6cada8ea"
        }
        Protocol::NostraMainnetUNOCollateral => {
            "0x07d717fb27c9856ea10068d864465a2a8f9f669f4f78013967de06149c09b9af"
        }
        Protocol::NostraMainnetUNOInterest => {
            "0x01325caf7c91ee415b8df721fb952fa88486a0fc250063eafddd5d3c67867ce7"
        }
        Protocol::NostraMainnetUNODebt => {
            "0x04b036839a8769c04144cc47415c64b083a2b26e4a7daa53c07f6042a0d35792"
        }
        Protocol::NostraMainnetUNOInterestCollateral => {
            "0x02a3a9d7bcecc6d3121e3b6180b73c7e8f4c5f81c35a90c8dd457a70a842b723"
        }
        Protocol::NostraMainnetNSTR => {
            "0x02b674ffda238279de5550d6f996bf717228d316555f07a77ef0a082d925b782"
        }
        Protocol::NostraMainnetNSTRCollateral => {
            "0x06f8ad459c712873993e9ffb9013a469248343c3d361e4d91a8cac6f98575834"
        }
        Protocol::NostraMainnetNSTRInterest => {
            "0x02589fc11f60f21af6a1dda3aeb7a44305c552928af122f2834d1c3b1a7aa626"
        }
        Protocol::NostraMainnetNSTRDebt => {
            "0x03e0576565c1b51fcac3b402eb002447f21e97abb5da7011c0a2e0b465136814"
        }
        Protocol::NostraMainnetNSTRInterestCollateral => {
            "0x046ab56ec0c6a6d42384251c97e9331aa75eb693e05ed8823e2df4de5713e9a4"
        }
        Protocol::NostraMainnetDAIV2 => {
            "0x0184dd6328115c2d5f038792e427f3d81d9552e40dd675e013ccbf74ba50b979"
        }
        Protocol::NostraMainnetDAIV2Interest => {
            "0x065bde349f553cf4bdd873e54cd48317eda0542764ebe5ba46984cedd940a5e4"
        }
        Protocol::NostraMainnetDAIV2Debt => {
            "0x06726ec97bae4e28efa8993a8e0853bd4bad0bd71de44c23a1cd651b026b00e7"
        }
        Protocol::Hashstack => "0x03dcf5c72ba60eb7b2fe151032769d49dd3df6b04fa3141dffd6e2aa162b7a6e",
        Protocol::Hashstack2 => {
            "0x01ef7f9f8bf01678dc6d27e2c26fb7e8eac3812a24752e6a1d6a49d153bec9f3"
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
