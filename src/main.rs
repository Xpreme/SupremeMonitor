use std::error::Error;
use std::env;
use dotenv::dotenv;

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
// Json Objects And Data

#[derive(Serialize, Deserialize)]
struct ProductIds {
    Accessories: Vec<u32>, 
    Bags: Vec<u32>,
    Hats: Vec<u32>,
    Jackets: Vec<u32>,
    Pants: Vec<u32>,
    Shirts: Vec<u32>,
    Shorts: Vec<u32>,
    Skate: Vec<u32>,
    Sweatshirts: Vec<u32>,
    Tops: Vec<u32>,
    New: Vec<u32>,
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Request to mobile endpoint

//TODO: Will I get banned from the mobile endpoint lol???
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();    
    pretty_env_logger::init();   

    // building client and request (has to use tls otherwise 304 status)
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut newstate: Vec<u64> = Vec::new();
    let mut oldstate: Vec<u64> = Vec::new();
    let webhookurl: hyper::Uri = env::var("webhookurl").unwrap().parse()?;
    let mobile_endpoint: hyper::Uri = "https://www.supremenewyork.com/mobile_stock.json".parse()?; 

    loop {
        //executing request
        //TODO: needs a loop | sorta done...

        let resp = client
            .get(mobile_endpoint.clone())
            .await?;
 
        //println!("status: {:#?}", resp.status());

        //going from stream to str
        let bod_byte = body::to_bytes(resp.into_body()).await?;
        let body = String::from_utf8(bod_byte.to_vec())
            .expect("resp not utf8");

        //so we can use this str as json
        let v: Value = serde_json::from_str(&body)?; 
     
        for (key, _value) in v["products_and_categories"].as_object().unwrap() {
            //println!("{}", key);
            for value in v["products_and_categories"][key].as_array().unwrap(){
                match value["id"].as_u64() {
                    None => println!("whoops invalid prod id!"),
                    Some(prodid) => newstate.push(prodid),
                }
            }
            println!("{:?}", newstate);
        }
    }
    //println!("body: {}", body);
}

///////////////////////////////////////////////////////////////////////////////////////////////////
// Variant Checking

///////////////////////////////////////////////////////////////////////////////////////////////////
// Notifs

async fn notify(url: hyper::Uri, price: String, style: String, Size: String)
-> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    
    let req = Request::builder()
        .method("POST")
        .uri(url)
        .header("Content-Type","application/json") 
        .body(());

    Ok(())
}


