use carmine_api_db;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let braavos = carmine_api_db::get_first_braavos_referrals();

    println!("{:#?}", braavos);
}
