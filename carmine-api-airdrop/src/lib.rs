pub mod merkle_tree;

#[cfg(test)]
mod tests {
    use crate::merkle_tree::{read_airdrop, MerkleTree};
    use starknet_crypto::{pedersen_hash, FieldElement};
    use std::str::FromStr;

    // mockup of a function that will be used in the SC
    // root will not be passed as an argument but stored in the SC
    fn cairo_root_generating(original_calldata: Vec<String>, root: FieldElement) -> bool {
        if original_calldata.len() < 3 {
            println!("Not enough arguments");
            return false;
        }

        let mut calldata: Vec<FieldElement> = original_calldata
            .to_vec()
            .iter()
            .map(|v| FieldElement::from_str(v).unwrap())
            .collect();
        let address = calldata.remove(0);
        let amount = calldata.remove(0);

        // leaf is hashed address and amount (base16)
        let mut hash_value = pedersen_hash(&address, &amount);

        println!("{}", hash_value);

        loop {
            if calldata.len() == 0 {
                break;
            }
            let next_hash = calldata.remove(0);
            if hash_value.lt(&next_hash) {
                hash_value = pedersen_hash(&hash_value, &next_hash);
            } else {
                hash_value = pedersen_hash(&next_hash, &hash_value);
            }
            println!("{}", hash_value);
        }
        println!("Root: {}", root);

        if hash_value.eq(&root) {
            println!("Sending {} to the address {}", amount, address);
            return true;
        } else {
            println!("Hacking attempt!");
            return false;
        }
    }

    #[test]
    fn valid_addresses() {
        let addresses: Vec<String> = read_airdrop().into_iter().map(|a| a.address).collect();
        let mt = MerkleTree::new();
        let root = mt.root.value.clone();

        for address in addresses.iter() {
            let calldata = mt
                .address_calldata(address)
                .expect("Failed getting calldata");
            assert!(cairo_root_generating(calldata, root.clone()));
        }
    }

    #[test]
    fn fail_with_random_addresses() {
        let addresses = vec!["0x123", "0xababcd"];
        let mt = MerkleTree::new();

        for address in addresses.iter() {
            assert!(mt.address_calldata(address).is_err());
        }
    }

    #[test]
    fn fail_with_calldata_tempering() {
        let addresses: Vec<String> = read_airdrop().into_iter().map(|a| a.address).collect();
        let mt = MerkleTree::new();
        let hacker_address =
            String::from("0x029AF9CF62C9d871453F3b033e514dc790ce578E0e07241d6a5feDF19cEEaF08");
        let root = mt.root.value.clone();

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
