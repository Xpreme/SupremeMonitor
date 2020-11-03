use hyper::Client;
use hyper::{ 
    Body,
    Method,
    Request,
    Uri
};
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
    let req = Request::builder()
        .method(Method::GET)
        .uri("https://www.supremenewyork.com/mobile_stock.json")
        .header(
            "User-Agent",
            "Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0 Mobile/15E148 Safari/604.1"
        )
        .body(Body::from(r#""#))?;


    let client = Client::new();
    //let mobile_endpoint = "https://www.supremenewyork.com/mobile_stock.json".parse()?;

    let resp = client
        .request(req)
        .await?;
 
   println!("body: {:#?}", resp.into_body());

//    match ::std::str::from_utf8(&resp.into_body().concat2()) {
//       Ok(_s) => println!("{}", _s),
//        Err(_err) => println!("{}", _err)
//    }

    Ok(())
}

