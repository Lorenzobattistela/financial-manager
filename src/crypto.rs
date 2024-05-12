use reqwest;
use std::env;
use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct BalanceResponse {
    result: String,
}

pub struct Bitcoin {
    address: String,
    balance: Option<f64>,
}

impl Bitcoin {
    pub fn new(address: String) -> Self {
        Bitcoin {
            address,
            balance: None,
        }
    }

    pub async fn get_bitcoin_balance(&mut self) -> Result<f64, Box<dyn std::error::Error>> {
        let url = format!("https://blockchain.info/q/addressbalance/{}", self.address);
        let response = reqwest::get(&url).await?;
        let balance: String = response.text().await?;
        let balance_btc: f64 = balance.parse().map_err(|e| {
            format!("Failed to parse balance: {}", e)
        })?;
        self.balance = Some(balance_btc / 1e8);
        Ok(balance_btc / 1e8)
    }
}

pub struct Ethereum {
    address: String,
    balance: Option<f64>,
}

impl Ethereum {
    pub fn new(address: String) -> Self {
        Ethereum {
            address,
            balance: None,
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
        let balance_wei: f64 = balance_response.result.parse().map_err(|e| {
            format!("Failed to parse balance: {}", e)
        })?;
        self.balance = Some(balance_wei / 1e18);
        Ok(balance_wei / 1e18)
    }
}

