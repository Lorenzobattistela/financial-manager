use reqwest;
use rocket::{http::hyper::request, serde::json::Json};
use serde::Deserialize;
use serde_json::Value;
use std::env;
#[derive(Deserialize, Debug)]
struct BalanceResponse {
    result: String,
}

pub struct Bitcoin {
    address: String,
    balance: Option<f64>,
    brl_balance: Option<f64>,
}

impl Bitcoin {
    pub fn new(address: String) -> Self {
        Bitcoin {
            address,
            balance: None,
            brl_balance: None,
        }
    }

    pub async fn get_bitcoin_balance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let url = format!("https://blockchain.info/q/addressbalance/{}", self.address);
        let response = reqwest::get(&url).await?;
        let balance: String = response.text().await?;
        let balance_btc: f64 = balance
            .parse()
            .map_err(|e| format!("Failed to parse balance: {}", e))?;

        let brl_price = self.get_price_brl().await?;
        self.brl_balance = Some((balance_btc / 1e8) * brl_price);
        self.balance = Some(balance_btc / 1e8);
        Ok(balance_btc / 1e8)
    }

    pub async fn get_price_brl(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=brl";
        let response = reqwest::get(url).await?;
        let json_str = response.text().await?;
        let json_value: Value = serde_json::from_str(&json_str).unwrap();
        let brl_value = json_value["bitcoin"]["brl"].as_f64().unwrap();
        Ok(brl_value)
    }
}

pub struct Ethereum {
    address: String,
    balance: Option<f64>,
    brl_balance: Option<f64>,
}

impl Ethereum {
    pub fn new(address: String) -> Self {
        Ethereum {
            address,
            balance: None,
            brl_balance: None,
        }
    }

    pub async fn get_ethereum_balance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let api_key = env::var("ETHERSCAN_API_KEY").expect("Etherscan API KEY should be set.");
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=balance&address={}&tag=latest&apikey={}",
            self.address, api_key
        );
        let response = reqwest::get(&url).await?;
        let balance_response: BalanceResponse = response.json().await?;
        let balance_wei: f64 = balance_response
            .result
            .parse()
            .map_err(|e| format!("Failed to parse balance: {}", e))?;

        let brl_price = self.get_price_brl().await?;

        self.brl_balance = Some((balance_wei / 1e18) * brl_price);
        self.balance = Some(balance_wei / 1e18);
        Ok(balance_wei / 1e18)
    }

    pub async fn get_price_brl(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=brl";
        let response = reqwest::get(url).await?;
        let json_str = response.text().await?;
        let json_value: Value = serde_json::from_str(&json_str).unwrap();
        let brl_value = json_value["ethereum"]["brl"].as_f64().unwrap();
        Ok(brl_value)
    }
}
