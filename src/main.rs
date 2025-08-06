use actix_web::{middleware::Logger, web, App, HttpServer, Responder, HttpResponse};
use cid::Cid;
use log::{debug, error, info};
use multihash_codetable::{Code, MultihashDigest};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

const SHA2_256: u64 = 0x12;

#[derive(Deserialize)]
struct DigestRequest {
    url: String,
}

#[derive(Serialize)]
struct DigestResponse {
    digest_multibase: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

async fn calculate_digest(req: Result<web::Json<DigestRequest>, actix_web::Error>) -> impl Responder {
    // Handle JSON deserialization errors
    let req = match req {
        Ok(req) => req,
        Err(e) => {
            error!("Request deserialization error: {}", e);
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "Missing required field: url".to_string(),
            });
        }
    };

    let client = reqwest::Client::new();
    let response = fetch_url(client, &req.url).await;
    let response = match response {
        Ok(resp) => resp,
        Err(e) => {
            error!("Failed to fetch URL {}: {}", req.url, e);
            return HttpResponse::InternalServerError().json(DigestResponse {
                digest_multibase: "Error fetching URL".to_string(),
            });
        }
    };

    let bytes = response.as_bytes();
    let digest = Code::Sha2_256.digest(bytes);
    let multihash = Cid::new_v1(SHA2_256, digest);
    let digest_multibase = match multihash.to_string_of_base(cid::multibase::Base::Base64) {
        Ok(base64) => base64,
        Err(e) => {
            error!("Failed to convert multihash to base64: {}", e);
            return HttpResponse::InternalServerError().json(DigestResponse {
                digest_multibase: "Error converting multihash to base64".to_string(),
            });
        }
    };

    info!(
        "Processed digest {} for URL: {}",
        &digest_multibase, &req.url
    );
    HttpResponse::Ok().json(DigestResponse { digest_multibase })
}

async fn fetch_url(client: Client, url: &str) -> Result<String, String> {
    debug!("Fetching URL: {}", url);
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status();
            debug!("Response status from {}: {}", url, status);

            if !status.is_success() {
                error!("HTTP error from {}: {}", url, status);
                return Err(format!("HTTP error: {}", status));
            }

            match resp.text().await {
                Ok(text) => {
                    debug!("Fetched URL: {}, content length: {} bytes", url, text.len());
                    Ok(text)
                }
                Err(e) => {
                    error!("Error reading content from {}: {}", url, e);
                    Err(format!("Error reading content: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Failed to fetch URL {}: {}", url, e);
            Err(format!("Error fetching URL: {}", e))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get host and port from environment variables or use defaults
    let host = env::var("LISTEN_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %r %s %b %T"))
            .route("/digest", web::post().to(calculate_digest))
    })
    .bind(&bind_address)?
    .run()
    .await
}