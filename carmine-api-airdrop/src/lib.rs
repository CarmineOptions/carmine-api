use merkle_tree::hash;
pub mod merkle_tree;

pub fn cairo_root_generating(original_calldata: Vec<String>, root: String) -> bool {
    if original_calldata.len() < 3 {
        println!("Not enough arguments");
        return false;
    }

    let mut calldata = original_calldata.to_vec();
    let address = calldata.remove(0);
    let amount = calldata.remove(0);

    // leaf is hashed address and amount (base16)
    let mut hash_value = hash(&address, &amount);

    loop {
        if calldata.len() == 0 {
            break;
        }
        let next_hash = calldata.remove(0);
        hash_value = hash(&hash_value, &next_hash);
    }

    if hash_value == root {
        let amount_base10 = u64::from_str_radix(amount.trim_start_matches("0x"), 16).unwrap();
        let formatted_address = format!("{}...{}", &address[..5], &address[59..]);
        println!(
            "Sending {} to the address {}",
            amount_base10, formatted_address
        );
        return true;
    } else {
        println!("Hacking attempt!");
        return false;
    }
}

#[cfg(test)]
mod tests {
    use crate::merkle_tree::{read_airdrop, MerkleTree};

    use super::*;

    #[test]
    fn ok_for_valid_addresses() {
        let addresses: Vec<String> = read_airdrop()
            .iter()
            .map(|a| a.address.to_string())
            .collect();
        let mt = MerkleTree::new();
        let root = mt.root.clone();

        for address in addresses.iter() {
            let calldata = mt
                .address_calldata(address)
                .expect("Failed getting calldata");
            assert!(cairo_root_generating(calldata, root.clone()));
        }
    }

    #[test]
    fn nok_for_random_addresses() {
        let addresses = vec!["0x123".to_string(), "0xababcd".to_string()];
        let mt = MerkleTree::new();

        for address in addresses.iter() {
            assert!(mt.address_calldata(address).is_none());
        }
    }

    #[test]
    fn nok_for_calldata_tempering() {
        let addresses: Vec<String> = read_airdrop()
            .iter()
            .map(|a| a.address.to_string())
            .collect();
        let mt = MerkleTree::new();
        let hacker_address =
            "0x029AF9CF62C9d871453F3b033e514dc790ce578E0e07241d6a5feDF19cEEaF08".to_string();
        let root = mt.root.clone();

        for address in addresses.iter() {
            let mut calldata = mt
                .address_calldata(address)
                .expect("Failed getting calldata");
            // temper with the valid calldata
            calldata[0] = hacker_address.clone();
            assert!(!cairo_root_generating(calldata, root.clone()));
        }
    }
}
