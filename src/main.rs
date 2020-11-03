use std::env;
use std::collections::HashMap;
use serde_json::{ Result, Value };

///////////////////////////////////////////////////////////////////////////////////////////////////
// Request to mobile endpoint

//TODO: Will I get banned from the mobile endpoint lol???

#[tokio::main]
async fn main() -> Result<()> {
    static MOBILE_AGENT: &str = concat!(
        "Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X)",
        "AppleWebKit/605.1.15 (KHTML, like Gecko)",
        "Version/12.0 Mobile/15E148 Safari/604.1",
);
    
    let client = reqwest::Client::builder()
        .user_agent(MOBILE_AGENT)
        .build()?;
    
    let resp = client::get("https://www.supremenewyork.com/mobile_stock.json")
        .send()
        .await?;

    let body: Value = serde_json::from_str(resp.text().await)?;
    println!("{}", body);

    Ok(())
}
