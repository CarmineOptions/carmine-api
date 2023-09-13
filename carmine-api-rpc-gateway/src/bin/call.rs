use carmine_api_rpc_gateway::{carmine_get_block_header, BlockTag};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let res = carmine_get_block_header(BlockTag::Number(200000)).await;

    if let Ok(v) = res {
        println!("{:#?}", v);
    }
}
