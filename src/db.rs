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
    // why use 'let' see:
    // [Cannot use temporaries as query! arguments when using SQLite](https://github.com/launchbadge/sqlx/issues/1430)
    let arg0 = status.skill_point.assign.e();
    let arg1 = status.skill_point.assign.t();
    let arg2 = status.skill_point.assign.w();
    let arg3 = status.skill_point.assign.f();
    let arg4 = status.skill_point.assign.a();
    let arg5 = status.skill_point.original.e();
    let arg6 = status.skill_point.original.t();
    let arg7 = status.skill_point.original.w();
    let arg8 = status.skill_point.original.f();
    let arg9 = status.skill_point.original.a();
    let arg10 = status.max_def.e();
    let arg11 = status.max_def.t();
    let arg12 = status.max_def.w();
    let arg13 = status.max_def.f();
    let arg14 = status.max_def.a();

    let arg15 = status.max_stat.mr();
    let arg16 = status.max_stat.ms();
    let arg17 = status.max_stat.spd();
    let arg18 = status.max_stat.ls();
    let arg19 = status.max_stat.hpr_raw();
    let arg20 = status.max_stat.hpr_pct();
    let arg21 = status.max_stat.sd_raw();
    let arg22 = status.max_stat.sd_pct();

    let arg23 = status.max_dam_pct.n();
    let arg24 = status.max_dam_pct.e();
    let arg25 = status.max_dam_pct.t();
    let arg26 = status.max_dam_pct.w();
    let arg27 = status.max_dam_pct.f();
    let arg28 = status.max_dam_pct.a();
    sqlx::query!(
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
                arg0,
                arg1,
                arg2,
                arg3,
                arg4,
                arg5,
                arg6,
                arg7,
                arg8,
                arg9,
                arg10,
                arg11,
                arg12,
                arg13,
                arg14,
                arg15,
                arg16,
                arg17,
                arg18,
                arg19,
                arg20,
                arg21,
                arg22,
                status.max_ehp,
                status.max_hp,
                status.max_hpr,
                arg23,
                arg24,
                arg25,
                arg26,
                arg28,
                arg27
            )
            .execute(&pool)
            .await
            .unwrap();
}
