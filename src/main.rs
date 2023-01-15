use anyhow::Result;
use reqwest::Client;
use dotenv::dotenv;
use std::env;

mod dto;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let oura_token = env::var("OURA_TOKEN")?;

    let client = Client::new();
    let url = "https://api.ouraring.com/v2/usercollection/daily_sleep";
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", oura_token))
        .send()
        .await?;
    let body = response.json::<dto::daily_sleep::DailySleepData>().await?;
    println!("{:?}", body);

    let client = Client::new();
    let url = "https://api.ouraring.com/v2/usercollection/sleep";
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", oura_token))
        .send()
        .await?;
    let body = response.json::<dto::sleep_periods::SleepPeriodsData>().await?;
    println!("{:?}", body);
    Ok(())
}
