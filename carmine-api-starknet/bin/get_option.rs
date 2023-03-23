use carmine_api_starknet::Carmine;

#[tokio::main]
async fn main() {
    let c = Carmine::new();
    let options = c.get_options_with_addresses().await;

    println!("Got {} options", options.len());
}
