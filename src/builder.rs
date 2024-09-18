mod config;
use config::build_config::*;
mod db;
use std::{
    borrow::BorrowMut,
    collections::VecDeque,
    fmt,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::Mutex;

use tokio::{runtime::Runtime, spawn, time::sleep};
use wynn_build_tools::*;

#[tokio::main]
async fn main() {
    let config = load_config("config/config.toml").await.unwrap();

    let (apparels, weapons) = load_from_json("config/items.json");
    let weapon = weapons
        .iter()
        .find(|v| v.name == config.items.weapon)
        .unwrap();

    let no_ring_apparels: [&[&Apparel]; 6] = [
        &find(&apparels[0], &config.items.helmets).unwrap(),
        &find(&apparels[1], &config.items.chest_plates).unwrap(),
        &find(&apparels[2], &config.items.leggings).unwrap(),
        &find(&apparels[3], &config.items.boots).unwrap(),
        &find(&apparels[5], &config.items.bracelets).unwrap(),
        &find(&apparels[6], &config.items.necklaces).unwrap(),
    ];

    let rings: [&[&Apparel]; 2] = [
        &find(&apparels[4], &config.items.rings).unwrap(),
        &find(&apparels[4], &config.items.rings).unwrap(),
    ];
    let ring_combinations = generate_no_order_combinations(rings[0].len());
    let total_combinations =
        no_ring_apparels.map(|f| f.len()).iter().product::<usize>() * ring_combinations.len();

    no_ring_apparels
        .iter()
        .for_each(|v| println!("{}:{}", v.first().unwrap().r#type, v.len()));
    println!("rings:{}", rings.first().unwrap().len());
    println!("total combinations: {}", total_combinations);

    let counter = Arc::new(AtomicUsize::new(0));
    let remaining_builds = Arc::new(AtomicUsize::new(total_combinations));
    let last_10_speeds = Arc::new(Mutex::new(VecDeque::with_capacity(10)));
    spawn_speed_watcher(
        counter.clone(),
        remaining_builds.clone(),
        ring_combinations.len(),
        last_10_speeds,
        total_combinations,
    );

    let db_pool = db::init().await;
    generate_full_combinations_with_random(
        1000,
        counter,
        &no_ring_apparels,
        |no_rings_combination| {
            let default = Default::default();
            let mut combination: [&Apparel; 8] = [&default; 8];
            combination[2..].copy_from_slice(&no_rings_combination);

            for indexes in &ring_combinations {
                let ring_combination = unsafe { select_from_arrays(&indexes, &rings) };
                combination[..2].copy_from_slice(&ring_combination);

                if let Ok(stat) = calculate_stats(&config, &combination, &weapon) {
                    let code = encode_build(
                        [
                            combination[2].id,
                            combination[3].id,
                            combination[4].id,
                            combination[5].id,
                            combination[0].id,
                            combination[1].id,
                            combination[6].id,
                            combination[7].id,
                        ],
                        config.player.lvl,
                        weapon.id,
                        [
                            stat.skill_point.original.e() as i32,
                            stat.skill_point.original.t() as i32,
                            stat.skill_point.original.w() as i32,
                            stat.skill_point.original.f() as i32,
                            stat.skill_point.original.a() as i32,
                        ],
                    );

                    let url = format!(
                        "{}{}{}",
                        config.hppeng.url_prefix, code, config.hppeng.url_suffix
                    );
                    if config.hppeng.log_builds {
                        println!("{}", url);
                        println!("{}", stat);
                    }

                    let rt = Runtime::new().unwrap();
                    rt.block_on(db::save_build(db_pool.clone(), url, stat, combination));
                };
            }
        },
        Option::Some(remaining_builds.clone()),
    );
    println!("done");
}

fn spawn_speed_watcher(
    counter: Arc<AtomicUsize>,
    remaining_builds: Arc<AtomicUsize>,
    coefficient: usize,
    mut last_10_speeds: Arc<Mutex<VecDeque<usize>>>,
    combinations: usize,
) {
    spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;

            let counter_val = counter.load(Ordering::Acquire);

            // Keep track of past 10 speeds and calculate the avg
            let speed = counter_val * coefficient;
            let mut last_10_speeds = last_10_speeds.borrow_mut().lock().await;

            // Remove 1 from 10 to see if we're nearly at capacity, then pop the last value
            if last_10_speeds.get(10 - 1).is_some() {
                last_10_speeds.pop_back();
            }
            last_10_speeds.push_front(speed);

            let mut remaining_time = usize::MAX;
            if last_10_speeds.get(0).is_some() {
                let avg_speed = last_10_speeds.iter().sum::<usize>() / last_10_speeds.len();
                if avg_speed > 0 {
                    remaining_time = combinations / avg_speed;
                }
            }

            // Uncommented because we're doing fetch_sub in SegmentedRandomNumbers's Iterator
            let remaining_builds_val = remaining_builds.load(Ordering::Acquire)/* - counter_val */;
            // remaining_builds.store(remaining_builds_val, Ordering::Release);

            println!("speed: {}/builds per second", speed);
            println!("remaining time: {}h left", remaining_time / 3600);
            println!("remaining builds: {}", remaining_builds_val);
            counter.store(0, Ordering::Release);
        }
    });
}

fn find<'a>(
    apparels: &'a Vec<Apparel>,
    names: &'a Vec<String>,
) -> Result<Vec<&'a Apparel>, Vec<&'a String>> {
    let result = names
        .iter()
        .map(|name| {
            let item = apparels.iter().find(|apparel| &apparel.name == name);
            match item {
                Some(v) => Ok(v),
                None => Err(name),
            }
        })
        .collect::<Vec<Result<_, _>>>();

    let (oks, errs): (Vec<_>, Vec<_>) = result.into_iter().partition(Result::is_ok);
    let ok_values: Vec<_> = oks.into_iter().map(Result::unwrap).collect();
    let err_values: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();

    if err_values.len() > 0 {
        Err(err_values)
    } else {
        Ok(ok_values)
    }
}
pub struct Status {
    pub max_stat: CommonStat,
    pub max_hpr: i32,
    pub max_hp: i32,
    pub max_ehp: i32,
    pub max_def: Point,
    pub skill_point: SkillPoints,
    pub max_dam_pct: Dam,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "max_stat:{}\nmax_hpr:{}\nmax_hp:{}\nmax_ehp:{}\nskill_point:\n{}\nmax_def:\t{}\nmax_dam_pct:\t{}",
            self.max_stat,
            self.max_hpr,
            self.max_hp,
            self.max_ehp,
            self.skill_point,
            self.max_def,
            self.max_dam_pct,
        )
    }
}

const MIN_16: i16 = i16::MIN / 2;
fn calculate_stats(
    config: &Config,
    combination: &[&Apparel; 8],
    weapon: &Weapon,
) -> Result<Status, String> {
    let max_hp = sum_hp_max(combination, weapon) + config.player.base_hp;
    if let Some(threshold) = &config.threshold_first {
        if let Some(v) = threshold.min_hp {
            if max_hp < v {
                return Err(format!(""));
            }
        }
    }

    let max_stat = CommonStat::sum_max_stats(combination, weapon);
    let max_hpr = max_stat.hpr();
    if let Some(threshold) = &config.threshold_second {
        let hpr_raw = threshold.min_hpr_raw.unwrap_or(MIN_16);
        let hpr_pct = threshold.min_hpr_pct.unwrap_or(MIN_16);
        let mr = threshold.min_mr.unwrap_or(MIN_16);
        let ls = threshold.min_ls.unwrap_or(MIN_16);
        let ms = threshold.min_ms.unwrap_or(MIN_16);
        let spd = threshold.min_spd.unwrap_or(MIN_16);
        let sd_raw = threshold.min_sd_raw.unwrap_or(MIN_16);
        let sd_pct = threshold.min_sd_pct.unwrap_or(MIN_16);

        if max_stat.any_lt(&CommonStat::new(
            hpr_raw, hpr_pct, mr, ls, ms, spd, sd_raw, sd_pct,
        )) {
            return Err(format!(""));
        }
        if let Some(v) = threshold.min_hpr {
            if max_hpr < v {
                return Err(format!(""));
            }
        }
    }

    let max_def = sum_def_max(combination, weapon);
    if let Some(threshold) = &config.threshold_third {
        let e = threshold.min_earth_defense.unwrap_or(MIN_16);
        let t = threshold.min_thunder_defense.unwrap_or(MIN_16);
        let w = threshold.min_water_defense.unwrap_or(MIN_16);
        let f = threshold.min_fire_defense.unwrap_or(MIN_16);
        let a = threshold.min_air_defense.unwrap_or(MIN_16);

        if max_def.any_lt(&Point::new(e, t, w, f, a)) {
            return Err(format!(""));
        }
    }

    let max_dam_pct = sum_dam_pct_max(combination, weapon);
    if let Some(threshold) = &config.threshold_fourth {
        let n = threshold.min_neutral_dam_pct.unwrap_or(MIN_16);
        let e = threshold.min_earth_dam_pct.unwrap_or(MIN_16);
        let t = threshold.min_thunder_dam_pct.unwrap_or(MIN_16);
        let w = threshold.min_water_dam_pct.unwrap_or(MIN_16);
        let f = threshold.min_fire_dam_pct.unwrap_or(MIN_16);
        let a = threshold.min_air_dam_pct.unwrap_or(MIN_16);

        if max_dam_pct.any_lt(&Dam::new(n, e, t, w, f, a)) {
            return Err(format!(""));
        }
    }

    if let Some(illegal_combinations) = &config.items.illegal_combinations {
        if is_illegal_combination(&combination, illegal_combinations.as_slice()) {
            return Err(format!(""));
        }
    }

    if SkillPoints::fast_gap(&combination) < -config.player.available_point {
        return Err(format!(""));
    }
    let (mut skill_point, _) = SkillPoints::full_put_calculate(combination);
    skill_point.add_weapon(weapon);

    if let Some(threshold) = &config.threshold_fifth {
        let e = threshold.min_earth_point.unwrap_or(MIN_16);
        let t = threshold.min_thunder_point.unwrap_or(MIN_16);
        let w = threshold.min_water_point.unwrap_or(MIN_16);
        let f = threshold.min_fire_point.unwrap_or(MIN_16);
        let a = threshold.min_air_point.unwrap_or(MIN_16);
        skill_point.assign(&Point::new(e, t, w, f, a));
    }

    if !skill_point.check(config.player.available_point) {
        return Err(format!(""));
    }

    let max_ehp = ehp(&skill_point, max_hp, &weapon.class);
    if let Some(threshold) = &config.threshold_fifth {
        if let Some(v) = threshold.min_ehp {
            if max_ehp < v {
                return Err(format!(""));
            }
        }
    }

    return Ok(Status {
        max_stat,
        max_hpr,
        max_hp,
        max_def,
        skill_point,
        max_ehp,
        max_dam_pct,
    });
}

fn is_illegal_combination(
    combination: &[&Apparel; 8],
    illegal_combinations: &[Vec<String>],
) -> bool {
    let names = combination.map(|v| &v.name);
    for illegal_combination in illegal_combinations {
        let mut count = 0;
        for name in names {
            if illegal_combination.contains(name) {
                count += 1;
            }
            if count > 1 {
                return true;
            }
        }
    }
    return false;
}
