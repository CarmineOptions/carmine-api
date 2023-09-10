use carmine_api_rpc_gateway::{carmine_amm_call, BlockTag, Entrypoint};

#[tokio::main]
async fn main() {
    let res = carmine_amm_call(
        Entrypoint::GetAllNonExpiredOptionsWithPremia,
        vec!["0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024".to_string()],
        BlockTag::Number(205000),
    )
    .await;
    println!("{:#?}", res);
}
