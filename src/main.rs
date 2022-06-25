use std::collections::HashMap;

#[tokio::main]
async fn main() {
    make_request("https://httpbin.org/ip").await.unwrap();
}

async fn make_request(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}",&url);
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
