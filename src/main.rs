#[tokio::main]
async fn main() {
    make_request("https://httpbin.org/ip").await.unwrap();
}

async fn make_request(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let acao_header = response
        .headers()
        .get("access-control-allow-origin")
        .unwrap();

    println!("-----------------=================------------------");
    println!("{:#?}", acao_header);

    Ok(())
}
