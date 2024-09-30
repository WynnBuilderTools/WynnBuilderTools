use std::io::Write;

use rand::Rng;
use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::fs::metadata;
use tokio::fs::{DirBuilder, File};

use crate::*;

pub async fn init(config: &Config) -> Pool<Sqlite> {
    // Create the database file if it doesn't exist
    if metadata(&config.hppeng.db_path).await.is_err() {
        let dirbuilder = DirBuilder::new();
        let db_folder_path = std::path::Path::new(&config.hppeng.db_path)
            .parent()
            .unwrap();

        if !db_folder_path.exists() {
            println!(
                "trying to create missing db folder at path: {}",
                db_folder_path.display()
            );
            dirbuilder
                .create(db_folder_path)
                .await
                .expect("tokio fs should be able to create missing db folder.");
        }

        println!(
            "trying to create missing data.db file at path: {}",
            &config.hppeng.db_path
        );
        File::create(&config.hppeng.db_path)
            .await
            .expect("tokio fs should be able to create missing data.db file.");

        println!(
            "Created missing data.db file at path: {}",
            &config.hppeng.db_path
        );
    }

    // Connect to the database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite:{}", config.hppeng.db_path))
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Expected a database file to exist at path {}",
                &config.hppeng.db_path
            )
        });

    // Run migrations
    let migrator = Migrator::new(std::path::Path::new(&config.hppeng.migrations_path))
        .await
        .expect("migrations folder should exist and contain a valid first migration");
    migrator.run(&pool).await.unwrap();

    // Sleep a couple seconds to let the migrations finish
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // Return pool
    pool
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

    let max_exp_bonus = status.max_exp_bonus;

    loop {
        let query = sqlx::query(
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
            max_air_dam_pct,
            max_exp_bonus
        ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6,
                    $7,
                    $8,
                    $9,
                    $10,
                    $11,
                    $12,
                    $13,
                    $14,
                    $15,
                    $16,
                    $17,
                    $18,
                    $19,
                    $20,
                    $21,
                    $22,
                    $23,
                    $24,
                    $25,
                    $26,
                    $27,
                    $28,
                    $29,
                    $30,
                    $31,
                    $32,
                    $33,
                    $34,
                    $35,
                    $36,
                    $37,
                    $38,
                    $39,
                    $40,
                    $41,
                    $42
                );
        "#,
        )
        .bind(url.clone())
        .bind(combination[2].name.clone())
        .bind(combination[3].name.clone())
        .bind(combination[4].name.clone())
        .bind(combination[5].name.clone())
        .bind(combination[0].name.clone())
        .bind(combination[1].name.clone())
        .bind(combination[6].name.clone())
        .bind(combination[7].name.clone())
        .bind(assign_strength)
        .bind(assign_dexterity)
        .bind(assign_intelligence)
        .bind(assign_defense)
        .bind(assign_agility)
        .bind(original_e)
        .bind(original_t)
        .bind(original_w)
        .bind(original_f)
        .bind(original_a)
        .bind(max_def_e)
        .bind(max_def_t)
        .bind(max_def_w)
        .bind(max_def_f)
        .bind(max_def_a)
        .bind(mr)
        .bind(ms)
        .bind(spd)
        .bind(ls)
        .bind(hpr_raw)
        .bind(hpr_pct)
        .bind(sd_raw)
        .bind(sd_pct)
        .bind(status.max_ehp)
        .bind(status.max_hp)
        .bind(status.max_hpr)
        .bind(max_dam_pct_n)
        .bind(max_dam_pct_e)
        .bind(max_dam_pct_t)
        .bind(max_dam_pct_w)
        .bind(max_dam_pct_f)
        .bind(max_dam_pct_a)
        .bind(max_exp_bonus);

        let result = query.execute(&pool).await;

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
                    println!("error on sql query: {}, retrying...", err);
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
