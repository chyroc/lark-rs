// use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct IPData {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp: IPData = reqwest::get("https://httpbin.org/ip")
        .await?
        .json()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
