use std::{fs::File, io::Write, path::Path};

use crate::{
    config::*,
    items::{ApiItems, Items},
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
        url = config.api.as_ref().unwrap_or(&defaults).url,
        version = config.api.as_ref().unwrap_or(&defaults).version,
        module = config.api.as_ref().unwrap_or(&defaults).module,
        query = config.api.as_ref().unwrap_or(&defaults).query,
    );

    let client = reqwest::Client::new();

    println!("fetching JSON from: {}...", request_url);
    let response_text = client
        .post(request_url)
        .body(r#"{"type": ["weapon", "armour", "accessory"]}"#)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("parsing items to api struct...");
    let parsed_json: ApiItems = match serde_json::from_str(&response_text) {
        Ok(json) => json,
        Err(err) => panic!("{}", err.to_string()),
    };

    println!("parsing items to hppeng struct...");
    let parsed_items: Items = parsed_json.into();

    println!("writing items to file...");
    // Open file for writing
    let mut file = File::create(&config.hppeng.items_file)
        .expect("fs should be able to create missing items file");

    // Write JSON string to file
    file.write_all(serde_json::to_string(&parsed_items).unwrap().as_bytes())
        .expect("fs should be able to write to items file");

    Ok(path)
}
