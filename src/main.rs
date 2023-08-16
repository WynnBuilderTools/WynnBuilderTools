mod config;
use config::*;
mod db;

use std::{
    fmt,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio::{runtime::Runtime, spawn, time::sleep};
use wynn_build_tools::*;

#[tokio::main]
async fn main() {
    let config = load_config("config/config.toml").await;

    let (apparels, weapons) = load_from_json("config/items.json");
    let weapon = weapons.iter().find(|v| v.name == config.weapon).unwrap();
    let no_ring_apparels: [&[&Apparel]; 6] = [
        &find(&apparels[0], &config.helmets).unwrap(),
        &find(&apparels[1], &config.chestplates).unwrap(),
        &find(&apparels[2], &config.leggings).unwrap(),
        &find(&apparels[3], &config.boots).unwrap(),
        &find(&apparels[5], &config.bracelets).unwrap(),
        &find(&apparels[6], &config.necklaces).unwrap(),
    ];
    let rings: [&[&Apparel]; 2] = [
        &find(&apparels[4], &config.rings).unwrap(),
        &find(&apparels[4], &config.rings).unwrap(),
    ];
    let ring_combinations = generate_no_order_combinations(rings[0].len());

    no_ring_apparels
        .iter()
        .for_each(|v| println!("{}:{}", v.first().unwrap().r#type, v.len()));
    println!("rings:{}", rings.first().unwrap().len());
    println!(
        "total combinations: {}",
        no_ring_apparels.map(|f| f.len()).iter().product::<usize>() * ring_combinations.len()
    );

    let counter = Arc::new(AtomicUsize::new(0));
    spawn_speed_watcher(counter.clone(), ring_combinations.len());

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
                        config.lvl,
                        weapon.id,
                        [
                            stat.skill_point.original.e() as i32,
                            stat.skill_point.original.t() as i32,
                            stat.skill_point.original.w() as i32,
                            stat.skill_point.original.f() as i32,
                            stat.skill_point.original.a() as i32,
                        ],
                    );

                    let url = format!("{}{}{}", config.url_refix, code, config.url_suffix);
                    println!("{}", url);
                    println!("{}", stat);

                    let rt = Runtime::new().unwrap();
                    rt.block_on(db::save_build(db_pool.clone(), url, stat, combination));
                };
            }
        },
    );
    println!("done");
}

fn spawn_speed_watcher(counter: Arc<AtomicUsize>, coefficient: usize) {
    spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;
            println!("speed:{}", counter.load(Ordering::Acquire) * coefficient);
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
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "max_stat:{}\nmax_hpr:{}\nmax_hp:{}\nmax_ehp:{}\nskill_point:\n{}\nmax_def:\t{}",
            self.max_stat, self.max_hpr, self.max_hp, self.max_ehp, self.skill_point, self.max_def
        )
    }
}

fn calculate_stats(
    config: &Config,
    combination: &[&Apparel; 8],
    weapon: &Weapon,
) -> Result<Status, String> {
    let max_hp = sum_hp_max(combination, weapon) + config.base_hp;
    if max_hp < config.threshold_hp {
        return Err(format!(""));
    }

    let max_stat = CommonStat::sum_max_stats(combination.as_slice(), weapon);
    if max_stat.any_lt(&config.threshold_stat) {
        return Err(format!(""));
    }

    let max_hpr = max_stat.hpr();
    if max_hpr < config.threshold_hpr {
        return Err(format!(""));
    }

    let max_def = sum_def_max(combination.as_slice(), weapon);
    if max_def.any_lt(&config.threshold_def) {
        return Err(format!(""));
    }

    if SkillPoints::fast_gap(&combination) < -config.available_point {
        return Err(format!(""));
    }
    let (mut skill_point, _) = SkillPoints::full_put_calculate(combination);
    skill_point.add_weapon(weapon);
    skill_point.assign(&config.threshold_point);
    if !skill_point.check(config.available_point) {
        return Err(format!(""));
    }

    let ehp = ehp(&skill_point, max_hp, &weapon.class);
    if config.threshold_ehp > ehp {
        return Err(format!(""));
    }

    return Ok(Status {
        max_stat,
        max_hpr,
        max_hp,
        max_def,
        skill_point,
        max_ehp: ehp,
    });
}
