use carmine_api_rpc_gateway::{carmine_get_block_header, BlockTag};

#[tokio::main]
async fn main() {
    // match carmine_amm_call(
    //     Entrypoint::GetValueOfPoolPosition,
    //     vec!["0x7aba50fdb4e024c1ba63e2c60565d0fd32566ff4b18aa5818fc80c30e749024".to_string()],
    //     BlockTag::Number(203180),
    // )
    // .await
    // {
    //     Ok(v) => {
    //         println!("SUCCESS: {:#?}", v);
    //     }
    //     Err(e) => {
    //         println!("FAIL: {:#?}", e);
    //     }
    // }
    let res = carmine_get_block_header(BlockTag::Number(200000)).await;

    if let Ok(v) = res {
        println!("{:#?}", v);
    }
}
