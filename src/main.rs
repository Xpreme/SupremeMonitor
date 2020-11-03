use std::error::Error;

use hyper::body;
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
use serde::{ 
    Deserialize,
    Serialize
};

use tokio;
///////////////////////////////////////////////////////////////////////////////////////////////////
// Json Objects

#[derive(Serialize, Deserialize)]
struct product_ids {
    acces: Vec<u32>, 
    bags: Vec<u32>,
    hats: Vec<u32>,
    jackets: Vec<u32>,
    pants: Vec<u32>,
    shirts: Vec<u32>,
    shorts: Vec<u32>,
    skate: Vec<u32>,
    sweatshirts: Vec<u32>,
    tops: Vec<u32>,
    new: Vec<u32>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Request to mobile endpoint

//TODO: Will I get banned from the mobile endpoint lol???
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    // building client and request (has to use tls otherwise 304 status)
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mobile_endpoint = "https://www.supremenewyork.com/mobile_stock.json".parse()?;

    //executing request
    //TODO: needs a loop

    let resp = client
        .get(mobile_endpoint)
        .await?;
 
    println!("status: {:#?}", resp.status());

    //going from stream to str
    let bod_byte = body::to_bytes(resp.into_body()).await?;
    let body = String::from_utf8(bod_byte.to_vec())
        .expect("resp not utf8");

    //so we can use this str as json
    let v: Value = serde_json::from_str(&body)?; 

    for (key, _value) in v["products_and_categories"].as_object().unwrap() {
        println!("{}", key);
        for (value) in v["products_and_categories"][key].as_array().unwrap(){
            println!("{}", value["id"]);
        }
    }

    println!("body: {}", body);

    Ok(())
}

