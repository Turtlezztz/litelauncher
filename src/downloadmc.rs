use reqwest;
use std::fs::File;

pub async fn download_json(url: String, filename: String ) {
    println!("Downloading {}", url);

    let mut resp = reqwest::get(url).await.expect("Download failed");
}