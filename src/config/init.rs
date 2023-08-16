use super::io;
use std::path::Path;
use wynn_build_tools::*;

const I16_MIN: i16 = i16::MIN / 2;
const I32_MIN: i32 = i32::MIN / 2;
pub struct Config {
    pub helmets: Vec<String>,
    pub chestplates: Vec<String>,
    pub leggings: Vec<String>,
    pub boots: Vec<String>,
    pub rings: Vec<String>,
    pub bracelets: Vec<String>,
    pub necklaces: Vec<String>,
    pub weapon: String,

    pub url_refix: String,
    pub url_suffix: String,

    pub available_point: i16,
    pub lvl: i32,
    pub base_hp: i32,

    pub threshold_hp: i32,
    pub threshold_ehp: i32,
    pub threshold_hpr: i32,
    pub threshold_point: Point,
    pub threshold_def: Point,
    pub threshold_stat: CommonStat,
}
pub async fn load_config(path: impl AsRef<Path>) -> Config {
    let config = io::load_config(path).await.unwrap();
    let threshold_ehp;
    let threshold_point;
    match config.threshold_fourth {
        Some(v) => {
            threshold_point = Point::new(
                v.min_earth_point.unwrap_or(I16_MIN),
                v.min_thunder_point.unwrap_or(I16_MIN),
                v.min_water_point.unwrap_or(I16_MIN),
                v.min_fire_point.unwrap_or(I16_MIN),
                v.min_air_point.unwrap_or(I16_MIN),
            );
            threshold_ehp = v.min_ehp.unwrap_or(I32_MIN);
        }
        None => {
            threshold_point = Point::new(I16_MIN, I16_MIN, I16_MIN, I16_MIN, I16_MIN);
            threshold_ehp = I32_MIN;
        }
    };
    let threshold_def: Point = match config.threshold_third {
        Some(value) => Point::new(
            value.min_earth_defense.unwrap_or(I16_MIN),
            value.min_thunder_defense.unwrap_or(I16_MIN),
            value.min_water_defense.unwrap_or(I16_MIN),
            value.min_fire_defense.unwrap_or(I16_MIN),
            value.min_air_defense.unwrap_or(I16_MIN),
        ),
        None => Point::new(I16_MIN, I16_MIN, I16_MIN, I16_MIN, I16_MIN),
    };

    let threshold_stat;
    let threshold_hpr;
    match config.threshold_second {
        Some(v) => {
            threshold_stat = CommonStat::new(
                v.min_hpr_raw.unwrap_or(I16_MIN),
                v.min_hpr_pct.unwrap_or(I16_MIN),
                v.min_mr.unwrap_or(I16_MIN),
                v.min_ls.unwrap_or(I16_MIN),
                v.min_ms.unwrap_or(I16_MIN),
                v.min_spd.unwrap_or(I16_MIN),
                v.min_sd_raw.unwrap_or(I16_MIN),
                v.min_sd_pct.unwrap_or(I16_MIN),
            );
            threshold_hpr = v.min_hpr.unwrap_or(I32_MIN);
        }
        None => {
            threshold_stat = CommonStat::new(
                I16_MIN, I16_MIN, I16_MIN, I16_MIN, I16_MIN, I16_MIN, I16_MIN, I16_MIN,
            );
            threshold_hpr = I32_MIN;
        }
    };

    let available_point: i16 = config.player.available_point;
    let lvl = config.player.lvl;
    let base_hp = config.player.base_hp;
    let threshold_hp = match config.threshold_first {
        Some(v) => v.min_hp.unwrap_or(I32_MIN),
        None => I32_MIN,
    };
    Config {
        helmets: config.items.helmets,
        chestplates: config.items.chestplates,
        leggings: config.items.leggings,
        boots: config.items.boots,
        rings: config.items.rings,
        bracelets: config.items.bracelets,
        necklaces: config.items.necklaces,
        weapon: config.items.weapon,

        url_suffix: config.hppeng.url_suffix,
        url_refix: config.hppeng.url_refix,

        available_point,
        lvl,
        base_hp,

        threshold_hp,
        threshold_ehp,
        threshold_hpr,
        threshold_point,
        threshold_def,
        threshold_stat,
    }
}
