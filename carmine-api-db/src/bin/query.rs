use carmine_api_db;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let res = carmine_api_db::get_insurance_events();

    println!("{:#?}", res);
}
