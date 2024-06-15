use carmine_api_fetcher::braavos::update_braavos_proscore;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    update_braavos_proscore().await;
}
