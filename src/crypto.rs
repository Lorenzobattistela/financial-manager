use reqwest;

pub async fn get_bitcoin_balance(address: &str) -> Result<f64, reqwest::Error> {
    let url = format!("https://blockchain.info/q/addressbalance/{}", address);
    let response = reqwest::get(&url).await?;
    let balance: String = response.text().await?;
    let balance_btc: f64 = balance.parse().unwrap();
    Ok(balance_btc / 1e8)
}

