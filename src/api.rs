#![allow(dead_code)]
#![allow(unused_variables)]


use crate::crypto::{Bitcoin, Ethereum};
use rocket::fs::TempFile;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use rocket::{get, post};
use rocket::serde::json::Json;


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
            key == "valid_api_key"
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
        }
    }
}


#[get("/bitcoin/balance/<address>")]
pub async fn bitcoin_balance(key: ApiKey<'_>, address: &str) -> Result<Json<String>, Status> {
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
pub async fn ethereum_balance(key: ApiKey<'_>, address: &str) -> Result<Json<String>, Status> {
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


#[post("/b3/parse", data = "<file>")]
pub async fn upload(mut file: TempFile<'_>) -> Result<Json<String>, std::io::Error> {
    file.copy_to(&"./tmp.xlsx").await?;
    let parsed_file = crate::b3::parse_file("./file.xlsx").expect("File should be parsed correctly.");
    let json_string = serde_json::to_string(&parsed_file).expect("Parsed file should be a valid json string.");
    Ok(Json(json_string))
}

