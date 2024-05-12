#![allow(dead_code)]
#![allow(unused_variables)]


use financial_manage::crypto::{Bitcoin, Ethereum};
use dotenv::dotenv;
#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::{get, launch, routes, Rocket, Build};

struct ApiKey<'r>(&'r str);

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct Json(String);

#[get("/bitcoin/balance/<address>")]
async fn bitcoin_balance(key: ApiKey<'_>, address: &str) -> Result<Json, Status> {
    let mut btc = Bitcoin::new(String::from(address));

    match btc.get_bitcoin_balance().await {
        Ok(balance) => {
            let json_res = format!("{{ \"balance\": {} }}", balance);
            Ok(Json(json_res))
        }
        Err(e) => {
            Err(Status::BadRequest)
        }
    }
}

#[get("/ethereum/balance/<address>")]
async fn ethereum_balance(key: ApiKey<'_>, address: &str) -> Result<Json, Status> {
    let mut eth = Ethereum::new(String::from(address));

    match eth.get_ethereum_balance().await {
        Ok(balance) => {
            let json_res = format!("{{ \"balance\": {} }}", balance);
            Ok(Json(json_res))
        }
        Err(e) => {
            Err(Status::BadRequest)
        }
    }
}



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
pub fn rocket() -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![sensitive])
        .mount("/", routes![bitcoin_balance])
        .mount("/", routes![ethereum_balance])
}