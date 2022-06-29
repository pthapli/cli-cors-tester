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

    // println!("access-control-allow-origin : {:#?}", acao_header);
    let acao = acao_header.to_str().unwrap();
    let _is_cors_allowed = match_headers(&acao);

    Ok(())
}

fn match_headers(acao: &str) -> bool {
    println!("access-control-allow-origin : {:#?}", acao);

    /**
     * @TODO 
     * Get the origin ip address from the request and check if we get 
     * the same response in access-control-allow-origin header 
     **/
 
    match acao {
        "*" => println!("No CORS error for you my friend"),
        _ => println!("Possible CORS, wait for the full code to be implemented"),
    }

    //Temporary
    return false;
}
