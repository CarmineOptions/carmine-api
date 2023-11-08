use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{self, BufReader, Write};

#[derive(Serialize, Deserialize, Debug)]
struct UserBalance {
    address: String,
    amount: String,
}

#[derive(Deserialize, Debug)]
struct NewUserBalance {
    address: String,
    amount: u128,
}

fn update_balances(first: &mut Vec<UserBalance>, second: Vec<NewUserBalance>) {
    for user_balance in second {
        if let Some(existing_balance) = first.iter_mut().find(|b| b.address == user_balance.address)
        {
            let prev_amount = existing_balance
                .amount
                .parse::<u128>()
                .expect("Failed parsing balance");

            let sum = prev_amount + user_balance.amount;
            existing_balance.amount = sum.to_string();
        } else {
            let new_entry = UserBalance {
                address: user_balance.address,
                amount: user_balance.amount.to_string(),
            };
            // Add new balance
            first.push(new_entry);
        }
    }
}

fn main() -> io::Result<()> {
    let file_path = "./carmine-api-airdrop/src/air-drop.json";

    // Open the file in read-only mode with buffer.
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let mut user_balances: Vec<UserBalance> = serde_json::from_reader(reader)?;

    let new_balances: Vec<NewUserBalance> = vec![
        NewUserBalance {
            address: "0x4d2FE1Ff7c0181a4F473dCd982402D456385BAE3a0fc38C49C0A99A620d1abe"
                .to_string(),
            amount: 2000,
        },
        NewUserBalance {
            address: "0x39e14d815587cdd5ae400684e5d60848d9a134b378260cc1f2de6e7aedcdb45"
                .to_string(),
            amount: 7700,
        },
        NewUserBalance {
            address: "0x639F7aD800Fcbe2aD56E3b000f9A0581759CcE989b3Ee09477055c0816A12c7"
                .to_string(),
            amount: 1500,
        },
        NewUserBalance {
            address: "0x6c67099b079c45213668939bcd120f3ce5ba44ecc3edc47eeee5c8c3a08d61".to_string(),
            amount: 4500,
        },
        NewUserBalance {
            address: "0x4d3E6A312d4089Ac798Ae3Cf5766AdB1c1863E23222B5602F19682E08DB2Bd1"
                .to_string(),
            amount: 4500,
        },
        NewUserBalance {
            address: "0x53eAD44Bb90853003d70E6930000Ef8C4a4819493fDC8f1CbdC1282121498eC"
                .to_string(),
            amount: 1500,
        },
        NewUserBalance {
            address: "0x30c3f654Ead1da0c9166d483d3dd436dcbB57Ce8E1AdaA129995103A8dcCA4D"
                .to_string(),
            amount: 20000,
        },
        NewUserBalance {
            address: "0x1fb62ac54f9fa99e1417f83bcb88485556427397f717ed4e7233bc99be31bff"
                .to_string(),
            amount: 20000,
        },
        NewUserBalance {
            address: "0x37080eb7d9ff1f71c143fa5ea125850756439af288982f828230835482708f9"
                .to_string(),
            amount: 6450,
        },
        NewUserBalance {
            address: "0x718505b87b5a448205ae22ac84a21b9e568b532ed95285c4c03973f8b1a73e8"
                .to_string(),
            amount: 48000,
        },
        NewUserBalance {
            address: "0x47991fc342a58b8446c7265b1657aa169ce0323b275dab0a06c8961bf481b37"
                .to_string(),
            amount: 5000,
        },
        NewUserBalance {
            address: "0x5462bd07DF5Cd8223ED55457DBACC34839f300eE7486aEdDdeB0976b465B911"
                .to_string(),
            amount: 6000,
        },
        NewUserBalance {
            address: "0x2af7135154dc27d9311b79c57ccc7b3a6ed74efd0c2b81116e8eb49dbf6aaf8"
                .to_string(),
            amount: 83334,
        },
        NewUserBalance {
            address: "0x2444d8f0a8c4562c9803ef25b2f6682b8d64c61e35c20031e23eb271a662fd8"
                .to_string(),
            amount: 30000,
        },
    ];

    const TEN_POW_18: u128 = 10_u128.pow(18);

    let sanitised_new_balances: Vec<NewUserBalance> = new_balances
        .into_iter()
        .map(|b| NewUserBalance {
            address: b.address.to_lowercase(),
            amount: b.amount * TEN_POW_18,
        })
        .collect();

    update_balances(&mut user_balances, sanitised_new_balances);

    let json_string = serde_json::to_string_pretty(&user_balances).expect("Failed to serialize");

    // Write the JSON string to a file
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(json_string.as_bytes())
        .expect("Failed to write to file");

    Ok(())
}
