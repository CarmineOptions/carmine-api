use carmine_api_starknet::Carmine;

#[tokio::main]
async fn main() {
    let c = Carmine::new();
    let res = c
        .get_option_info_from_addresses(
            "0x4be31a0425cba0a20694c66079b54a98d9e242f11522382c872ac32915ea929",
        )
        .await;

    if let Ok(option) = res {
        println!("Got option: {:?}", option);
    }
}
