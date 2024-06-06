use carmine_api_db::lp_value::update_lp_prices;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();
    update_lp_prices();
}
