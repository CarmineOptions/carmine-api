use crate::constants::MATH_64;

pub fn strike_from_hex(hex_str: &str) -> f64 {
    // Parse the hex string as u128, skipping the "0x" prefix
    let num = u128::from_str_radix(&hex_str[2..], 16).expect("Failed to parse hex string");
    num as f64 / MATH_64
}

#[cfg(test)]
mod tests {
    use crate::utils::strike_from_hex;

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
}
