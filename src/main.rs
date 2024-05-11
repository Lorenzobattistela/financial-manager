// #[macro_use] extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }
use financial_manage::crypto::{Bitcoin, Ethereum};
use financial_manage::b3::parse_file;
use std::env;
use dotenv::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();

    let parsed = parse_file("test.xlsx").unwrap();
    println!("{:?}", parsed);

    let address = "bc1qup32v2aazd6k7xx5d5dwxtuu8axeam68rwnazj"; // Replace with the desired Bitcoin address
    let mut btc = Bitcoin::new(address.to_string());
    match btc.get_bitcoin_balance().await {
        Ok(balance) => {
            // let balance_btc = balance as f64 / 1e8; // Convert satoshis to BTC
            println!("Balance of {}: {} BTC", address, balance);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    let api_key = env::var("ETHERSCAN_API_KEY").expect("Etherscan API KEY should be set.");
    let eth_addr = "0x702879Dc9CE3a526d51f21a8788EFb1B708911d1";

    let mut eth = Ethereum::new(eth_addr.to_string());
    match eth.get_ethereum_balance(&api_key).await {
        Ok(balance) => {
            println!("Balance of {}: {} ETH", eth_addr, balance);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}