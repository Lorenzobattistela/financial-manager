#![allow(dead_code)]
#![allow(unused_variables)]

#[macro_use] extern crate rocket;

use dotenv::dotenv;
use rocket::{launch, routes, Rocket, Build};
use financial_manage::api::{ethereum_balance, bitcoin_balance, upload};


#[launch]
pub fn rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![bitcoin_balance])
        .mount("/", routes![ethereum_balance])
        .mount("/", routes![upload])
}