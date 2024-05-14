#![allow(dead_code)]
#![allow(unused_variables)]


use crate::crypto::{Bitcoin, Ethereum};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::{get, post};
use rocket::serde::json::Json;
use std::env;


pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            let api_key = env::var("API_KEY").expect("Our API_KEY should be set.");
            key == &api_key
        }

        match req.headers().get_one("x_api_key") {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}


#[get("/bitcoin/balance/<address>")]
pub async fn bitcoin_balance(key: ApiKey<'_>, address: &str) -> Result<Json<String>, (Status, String)> {
    if address.is_empty() {
        return Err((Status::BadRequest, String::from("BTC address is empty")));
    }

    let mut btc = Bitcoin::new(String::from(address));

    match btc.get_bitcoin_balance().await {
        Ok(balance) => {
            let json_res = format!("{{ \"balance\": {} }}", balance);
            Ok(Json(json_res))
        }
        Err(e) => {
            Err((Status::BadRequest, String::from("Unable to get BTC balance from adress.")))
        }
    }
}

#[get("/ethereum/balance/<address>")]
pub async fn ethereum_balance(key: ApiKey<'_>, address: &str) -> Result<Json<String>, (Status, String)> {
    if address.is_empty() {
        return Err((Status::BadRequest, String::from("ETH address is empty")));
    }
    let mut eth = Ethereum::new(String::from(address));

    match eth.get_ethereum_balance().await {
        Ok(balance) => {
            let json_res = format!("{{ \"balance\": {} }}", balance);
            Ok(Json(json_res))
        }
        Err(e) => {
            Err((Status::BadRequest, String::from("Unable to get ETH balance from address")))
        }
    }
}


#[post("/b3/parse", data = "<file>")]
pub async fn upload(key: ApiKey<'_>, mut file: TempFile<'_>) -> Result<Json<String>, std::io::Error> {
    let tmp_path = "./tmp.xlsx";
    file.copy_to(&tmp_path).await?;
    let parsed_file = crate::b3::parse_file(&tmp_path).expect("File should be parsed correctly.");
    let json_string = serde_json::to_string(&parsed_file).expect("Parsed file should be a valid json string.");

    match std::fs::remove_file(tmp_path) {
        Ok(_) => println!("File deleted successfully"),
        Err(e) => eprintln!("Failed to delete file: {}", e),
    }

    Ok(Json(json_string))
}

