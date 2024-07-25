use std::str::FromStr;

use starknet::core::types::FieldElement;

use crate::{constants::MATH_64, types::PriceResponse};

pub fn strike_from_hex(hex_str: &str) -> f64 {
    // Parse the hex string as u128, skipping the "0x" prefix
    let num = u128::from_str_radix(&hex_str[2..], 16).expect("Failed to parse hex string");
    num as f64 / MATH_64
}

pub async fn get_coingecko_prices() -> Result<PriceResponse, reqwest::Error> {
    let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum,usd-coin,starknet,bitcoin&vs_currencies=usd";

    reqwest::get(url).await?.json::<PriceResponse>().await
}

pub fn normalize_address(address: &str) -> String {
    let res = &address[2..].trim_start_matches('0');
    format!("0x{}", res)
}

pub fn felt_to_float(felt: FieldElement, decimals: usize) -> f64 {
    let input_str = felt.to_string(); // decimal number as string

    // Ensure the string is long enough
    let padded_str = if input_str.len() < decimals {
        format!("{:0>width$}", input_str, width = decimals + 1)
    } else {
        input_str.to_string()
    };

    // Insert the decimal point 18 positions from the right
    let len = padded_str.len();
    let decimal_str = format!(
        "{}.{}",
        &padded_str[..len - decimals],
        &padded_str[len - decimals..]
    );

    let result: f64 = decimal_str.parse().expect("Failed parsing hex string");

    result
}

pub fn string_to_float(str_num: &str, decimals: usize) -> f64 {
    let felt = FieldElement::from_str(str_num).expect("Failed parsing felt");

    felt_to_float(felt, decimals)
}

pub fn hex_to_u128(str_num: &str) -> u128 {
    u128::from_str_radix(&str_num[2..], 16).expect("Failed to parse hex to u128")
}

pub fn tokens_to_usd(tokens: &str, decimals: usize, price: f32) -> f32 {
    let tokens_f64 = string_to_float(tokens, decimals);
    (tokens_f64 as f32) * price
}

#[cfg(test)]
mod tests {
    use crate::utils::{strike_from_hex, string_to_float};

    #[test]
    fn float_strike_price() {
        assert_eq!(strike_from_hex("0x23333333333334000"), 2.2);
    }
    #[test]
    fn int_strike_price() {
        assert_eq!(strike_from_hex("0xc1c0000000000000000"), 3100.0);
    }
    #[test]
    fn high_strike_price() {
        assert_eq!(strike_from_hex("0xb3b00000000000000000"), 46000.0);
    }

    #[test]
    fn float_from_hex() {
        assert_eq!(string_to_float("0x2DFD1C040", 9), 12.345);
        assert_eq!(string_to_float("0xe32ec9d196c2cbd", 18), 1.0231402248470642);
    }

    #[test]
    fn float_from_dec() {
        assert_eq!(string_to_float("1234567890000000000", 18), 1.23456789);
    }
}
