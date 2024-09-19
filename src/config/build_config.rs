use serde::Deserialize;
use std::{path::Path, str};
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub items: Items,
    pub player: Player,
    pub hppeng: Hppeng,
    pub threshold_first: Option<ThresholdFirst>,
    pub threshold_second: Option<ThresholdSecond>,
    pub threshold_third: Option<ThresholdThird>,
    pub threshold_fourth: Option<ThresholdFourth>,
    pub threshold_fifth: Option<ThresholdFifth>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Items {
    pub helmets: Vec<String>,
    pub chest_plates: Vec<String>,
    pub leggings: Vec<String>,
    pub boots: Vec<String>,
    pub rings: Vec<String>,
    pub bracelets: Vec<String>,
    pub necklaces: Vec<String>,
    pub weapon: String,
    pub illegal_combinations: Option<Vec<Vec<String>>>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Player {
    pub lvl: i32,
    pub available_point: i16,
    pub base_hp: i32,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Hppeng {
    pub url_prefix: String,
    pub url_suffix: String,
    pub log_builds: bool,
    pub log_db_errors: bool,
    pub db_retry_count: u8,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ThresholdFirst {
    pub min_hp: Option<i32>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ThresholdSecond {
    pub min_hpr_raw: Option<i16>,
    pub min_hpr_pct: Option<i16>,
    pub min_mr: Option<i16>,
    pub min_ls: Option<i16>,
    pub min_ms: Option<i16>,
    pub min_spd: Option<i16>,
    pub min_sd_raw: Option<i16>,
    pub min_sd_pct: Option<i16>,

    pub min_hpr: Option<i32>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ThresholdThird {
    pub min_earth_defense: Option<i16>,
    pub min_thunder_defense: Option<i16>,
    pub min_water_defense: Option<i16>,
    pub min_fire_defense: Option<i16>,
    pub min_air_defense: Option<i16>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ThresholdFourth {
    pub min_neutral_dam_pct: Option<i16>,
    pub min_earth_dam_pct: Option<i16>,
    pub min_thunder_dam_pct: Option<i16>,
    pub min_water_dam_pct: Option<i16>,
    pub min_fire_dam_pct: Option<i16>,
    pub min_air_dam_pct: Option<i16>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ThresholdFifth {
    pub min_earth_point: Option<i16>,
    pub min_thunder_point: Option<i16>,
    pub min_water_point: Option<i16>,
    pub min_fire_point: Option<i16>,
    pub min_air_point: Option<i16>,

    pub min_ehp: Option<i32>,
}

pub async fn load_config(path: impl AsRef<Path>) -> Result<Config, String> {
    let mut f = match File::open(path).await {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }?;
    let mut buffer = Vec::new();

    match f.read_to_end(&mut buffer).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }?;

    match toml::from_str(str::from_utf8(&buffer).unwrap()) {
        Ok(ok) => Ok(ok),
        Err(err) => Err(err.to_string()),
    }
}
