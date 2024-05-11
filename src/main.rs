// #[macro_use] extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }
use financial_manage::crypto::get_bitcoin_balance;
use financial_manage::b3::parse_file;
use calamine::{Reader, open_workbook, Xlsx, Data};

// pub fn main() {
//     // let parsed = parse_file("test.xlsx").unwrap();
//     // println!("{:?}", parsed);
//     let client = init_rpc_client();
//     println!("{:?}", client);
//     let a = client.get_block_count().unwrap();
//     println!("{:?}", a);
// }

#[tokio::main]
async fn main() {
    let address = "bc1qup32v2aazd6k7xx5d5dwxtuu8axeam68rwnazj"; // Replace with the desired Bitcoin address
    match get_bitcoin_balance(address).await {
        Ok(balance) => {
            // let balance_btc = balance as f64 / 1e8; // Convert satoshis to BTC
            println!("Balance of {}: {} BTC", address, balance);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}