use std::error::Error;

use hyper::body;
use tokio;
use hyper::{ 
    Client,
    Body,
    Method,
    Request,
    Uri
};
use hyper_tls::HttpsConnector;
use serde_json::{ 
    Result,
    Value
};
//use serde::de;

///////////////////////////////////////////////////////////////////////////////////////////////////
// Request to mobile endpoint

//TODO: Will I get banned from the mobile endpoint lol???
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mobile_endpoint = "https://www.supremenewyork.com/mobile_stock.json".parse()?;

    let resp = client
        .get(mobile_endpoint)
        .await?;
 
    println!("status: {:#?}", resp.status());

    let bod_byte = body::to_bytes(resp.into_body()).await?;
    let body = String::from_utf8(bod_byte.to_vec())
        .expect("resp not utf8");

    println!("body: {}", body);

    Ok(())
}

