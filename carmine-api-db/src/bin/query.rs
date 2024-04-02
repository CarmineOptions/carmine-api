use carmine_api_db::get_votes;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let votes = get_votes();
    println!("{:#?}", votes);
}
