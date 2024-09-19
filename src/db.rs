use std::io::Write;

use rand::Rng;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

use crate::*;

pub async fn init() -> Pool<Sqlite> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite:db/data.db")
        .await
        .unwrap()
}

pub async fn save_build(
    pool: Pool<Sqlite>,
    url: String,
    status: Status,
    combination: [&Apparel; 8],
) {
    let config = load_config("config/config.toml").await.unwrap();

    // why use 'let' see:
    // [Cannot use temporaries as query! arguments when using SQLite](https://github.com/launchbadge/sqlx/issues/1430)
    let assign_strength = status.skill_point.assign.e();
    let assign_dexterity = status.skill_point.assign.t();
    let assign_intelligence = status.skill_point.assign.w();
    let assign_defense = status.skill_point.assign.f();
    let assign_agility = status.skill_point.assign.a();

    let original_e = status.skill_point.original.e();
    let original_t = status.skill_point.original.t();
    let original_w = status.skill_point.original.w();
    let original_f = status.skill_point.original.f();
    let original_a = status.skill_point.original.a();

    let max_def_e = status.max_def.e();
    let max_def_t = status.max_def.t();
    let max_def_w = status.max_def.w();
    let max_def_f = status.max_def.f();
    let max_def_a = status.max_def.a();

    let mr = status.max_stat.mr();
    let ms = status.max_stat.ms();
    let spd = status.max_stat.spd();
    let ls = status.max_stat.ls();
    let hpr_raw = status.max_stat.hpr_raw();
    let hpr_pct = status.max_stat.hpr_pct();
    let sd_raw = status.max_stat.sd_raw();
    let sd_pct = status.max_stat.sd_pct();

    let max_dam_pct_n = status.max_dam_pct.n();
    let max_dam_pct_e = status.max_dam_pct.e();
    let max_dam_pct_t = status.max_dam_pct.t();
    let max_dam_pct_w = status.max_dam_pct.w();
    let max_dam_pct_f = status.max_dam_pct.f();
    let max_dam_pct_a = status.max_dam_pct.a();

    loop {
        let result = sqlx::query!(
            r#"
        INSERT INTO build (
            url,
            helmet,
            chest_plate,
            leggings,
            boots,
            ring_1,
            ring_2,
            bracelet,
            necklace,
            earth_assign,
            thunder_assign,
            water_assign,
            fire_assign,
            air_assign,
            earth_original,
            thunder_original,
            water_original,
            fire_original,
            ari_original,
            earth_def,
            thunder_def,
            water_def,
            fire_def,
            air_def,
            max_mr,
            max_ms,
            max_spd,
            max_ls,
            max_hpr_raw,
            max_hpr_pct,
            max_sd_raw,
            max_sd_pct,
            max_ehp,
            max_hp,
            max_hpr,
            max_neutral_dam_pct,
            max_earth_dam_pct,
            max_thunder_dam_pct,
            max_water_dam_pct,
            max_fire_dam_pct,
            max_air_dam_pct
        ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24,?25,?26,?27,?28,?29,?30,?31,?32,?33,?34,?35,?36,?37,?38,?39,?40,?41);
        "#,
            url,
            combination[2].name,
            combination[3].name,
            combination[4].name,
            combination[5].name,
            combination[0].name,
            combination[1].name,
            combination[6].name,
            combination[7].name,
            assign_strength,
            assign_dexterity,
            assign_intelligence,
            assign_defense,
            assign_agility,
            original_e,
            original_t,
            original_w,
            original_f,
            original_a,
            max_def_e,
            max_def_t,
            max_def_w,
            max_def_f,
            max_def_a,
            mr,
            ms,
            spd,
            ls,
            hpr_raw,
            hpr_pct,
            sd_raw,
            sd_pct,
            status.max_ehp,
            status.max_hp,
            status.max_hpr,
            max_dam_pct_n,
            max_dam_pct_e,
            max_dam_pct_t,
            max_dam_pct_w,
            max_dam_pct_f,
            max_dam_pct_a
        )
        .execute(&pool)
        .await;

        let mut retry_count = 0;

        let mut rng = rand::thread_rng();

        // Much cleaner than what we had before
        match result {
            Ok(_) => break,
            Err(err) => {
                // Log error to an error_log file
                let mut file = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("db/db_error.log")
                    .unwrap();
                let error = format!("{}\n", err);
                file.write_all(error.as_bytes()).unwrap();

                if config.hppeng.log_db_errors {
                    println!("error on SQL query, retrying...");
                }

                retry_count += 1;
                if retry_count > config.hppeng.db_retry_count {
                    // Maximum retries defined in config
                    if config.hppeng.log_db_errors {
                        println!(
                            "max retries exceeded. giving up on writing to db.\nerror: {:?}",
                            err
                        );
                    }
                    break;
                }

                // Exponential backoff with jitter
                let base_wait_ms = 2u64.pow(retry_count.into()) * 100; // 100ms, 200ms, 400ms...
                let jitter = rng.gen_range(0..100); // Add random jitter
                let wait_duration = Duration::from_millis(base_wait_ms + jitter);
                tokio::time::sleep(wait_duration).await;
            }
        }
    }
}
