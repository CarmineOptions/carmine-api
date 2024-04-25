use carmine_api_db::get_pool_tvl_map;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let pool_tvl_map = get_pool_tvl_map();

    println!("{:#?}", pool_tvl_map);
}
