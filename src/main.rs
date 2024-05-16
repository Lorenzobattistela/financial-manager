#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use financial_manage::api::{
    bitcoin_balance, ethereum_balance, get_btc_brl_price, get_eth_brl_price, upload,
};
use rocket::{launch, routes, Build, Rocket};

#[launch]
pub fn rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![bitcoin_balance])
        .mount("/", routes![ethereum_balance])
        .mount("/", routes![get_btc_brl_price])
        .mount("/", routes![get_eth_brl_price])
        .mount("/", routes![upload])
}

