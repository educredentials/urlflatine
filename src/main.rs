use actix_web::{web, App, HttpServer, Responder};
use multihash_codetable::{Code, MultihashDigest};
use cid::Cid;
use std::convert::TryFrom;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const SHA2_256: u64 = 0x12;

#[derive(Deserialize)]
struct DigestRequest {
    url: String,
}

#[derive(Serialize)]
struct DigestResponse {
    digest_multibase: String,
}

async fn calculate_digest(req: web::Json<DigestRequest>) -> impl Responder {
    let client = reqwest::Client::new();
    let response = fetch_url(client, &req.url).await;
    let response = match response {
        Ok(resp) => resp,
        Err(_) => return web::Json(DigestResponse { digest_multibase: "Error fetching URL".to_string() }),
    };
    
    let bytes = response.as_bytes();
    let digest = Code::Sha2_256.digest(bytes);
    let multihash = Cid::new_v1(SHA2_256, digest);
    let digest_multibase = match Cid::try_from(multihash.to_bytes()) {
        Ok(cid) => cid.to_string(),
        Err(_) => "Error converting to multibase".to_string(),
    };
    
    web::Json(DigestResponse { digest_multibase })
}

async fn fetch_url(client: Client, url: &str) -> Result<String, String> {
    match client.get(url).send().await {
        Ok(resp) => match resp.text().await {
            Ok(text) => {
                println!("Fetched URL: {}", url);
                println!("Content length: {}", text.len());
                Ok(text)
            },
            Err(_) => Err("Error reading content".to_string()),
        },
        Err(_) => Err("Error fetching URL".to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/digest", web::post().to(calculate_digest))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await 
}