use std::collections::HashMap;

#[tokio::main]
async fn main() {
    make_request("https://www.rust-lang.org").await.unwrap();
}

async fn make_request(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let acao_header = response
        .headers()
        .get("access-control-allow-origin")
        .unwrap();

    // Get the ip address and origin value of the request header

    let origin_ip = get_origin_address().await;

    println!("origin ip : {}", origin_ip);

    let acao = acao_header.to_str().unwrap();

    //original code
    let _is_cors_allowed = match_headers(&acao, &origin_ip);

    //test
    // let _is_cors_allowed = match_headers("http://google.com", "http://google.com");

    Ok(())
}

fn match_headers(acao: &str, origin_ip: &str) -> bool {
    println!("access-control-allow-origin : {:#?}", acao);
    println!("origin_ip : {}", origin_ip);
    match acao {
        "*" => println!("No CORS error for you my friend"),
        "test" => println!("This is just for testing"),
        _ if acao.to_string() == origin_ip.to_string() => {
            println!("Origin IP matches. No error for you my friend")
        }
        _ => println!("Possible CORS, wait for the full code to be implemented"),
    }

    //Temporary
    return false;
}

async fn get_origin_address() -> String {
    let find_origin_url = "https://httpbin.org/ip";

    let response = reqwest::get(find_origin_url).await.unwrap();

    let ip_hash_map = response.json::<HashMap<String, String>>().await.unwrap();

    println!("This better work {:?}", ip_hash_map.get("origin"));

    let origin = ip_hash_map.get("origin").unwrap();

    println!("origin value bitch -> {}", origin);

    origin.to_string()
}
