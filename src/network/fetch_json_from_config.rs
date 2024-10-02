use std::{fs::File, io::Write, path::Path};
use serde_json::Value;

use crate::{
    build_config::Api,
    config::build_config::Config,
};

pub async fn fetch_json_from_config<P>(path: P, config: &Config) -> Result<impl AsRef<Path>, &str>
where
    P: AsRef<Path>,
{
    let defaults = Api {
        url: "https://api.wynncraft.com".to_string(),
        version: "v3".to_string(),
        module: "item".to_string(),
        query: "search?fullResult".to_string(),
    };

    let request_url = format!(
        "{url}/{version}/{module}/{query}",
        url = config.api.as_ref().unwrap_or_else(|| { &defaults }).url,
        version = config.api.as_ref().unwrap_or_else(|| { &defaults }).version,
        module = config.api.as_ref().unwrap_or_else(|| { &defaults }).module,
        query = config.api.as_ref().unwrap_or_else(|| { &defaults }).query,
    );

    let client = reqwest::Client::new();
    let response_text = client
        .post(request_url)
        .body(
            r#"{"type": ["weapon", "armour", "accessory"]}"#,
        )
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let parsed_json: Value = serde_json::from_str(&response_text)
        .expect("failed to parse json response");

    println!("{:#?}", parsed_json);

    // Open file for writing
    let mut file = File::create(&config.hppeng.items_file)
        .expect("fs should be able to create missing items file");

    // Write JSON string to file
    file.write_all(response_text.as_bytes())
        .expect("failed to write json to file");

    Ok(path)
}
