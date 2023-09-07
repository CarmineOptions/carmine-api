use carmine_api_rpc_gateway::{carmine_amm_call, Entrypoint};

#[tokio::main]
async fn main() {
    let res = carmine_amm_call(Entrypoint::GetAllNonExpiredOptionsWithPremia, vec![]).await;
    println!("{:#?}", res);
}
