mod db;

use std::fmt::Write;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    fs::File,
    io::BufReader,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use tokio::{runtime::Runtime, spawn, time::sleep};

use wynn_build_tools::calculate::*;
use wynn_build_tools::config::*;
use wynn_build_tools::items::*;
use wynn_build_tools::network::*;
use wynn_build_tools::util::*;

const SPLIT_STR: &str = ".";

#[tokio::main]
async fn main() {
    let config = load_config("config/config.toml").await.unwrap();
    let hppeng_codes: HppengCodes = HppengCodes::split_hppeng_url(&config.hppeng.template_url);

    let (apparels, weapons) = match load_from_json(&config.hppeng.items_file) {
        Ok(v) => v,
        Err(_) => {
            let api_fetch_attempt =
                fetch_json_from_config(&config.hppeng.items_file, &config).await;

            let new_path = match api_fetch_attempt {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            };

            let second_attempt = load_from_json(&new_path);

            match second_attempt {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            }
        }
    };

    let weapon = weapons
        .iter()
        .find(|v| v.name == config.items.weapon)
        .unwrap();

    let abilities = load_abilities();
    let active_abilities = decode_atree(
        &abilities.get(&Class::from(weapon)).unwrap(),
        &hppeng_codes.ability,
    );
    let (common_stat, dam_raw, dam_pct, dam_add, mut spells) = atree_merge(&active_abilities);
    validate_config_damages(&spells, &config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    retain_spells(&mut spells, &config);

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
    spawn_speed_watcher(counter.clone(), ring_combinations.len(), total_combinations);

    let db_pool = db::init(&config).await;
    generate_full_combinations_with_random(
        1000,
        counter,
        &no_ring_apparels,
        |no_rings_combination| {
            let default = Default::default();
            let mut combination: [&Apparel; 8] = [&default; 8];
            combination[2..].copy_from_slice(&no_rings_combination);

            for indexes in &ring_combinations {
                let ring_combination = unsafe { select_from_arrays(indexes, &rings) };
                combination[..2].copy_from_slice(&ring_combination);

                if let Ok(stat) = calculate_stats(
                    &config,
                    &common_stat,
                    &dam_pct,
                    dam_raw,
                    &dam_add,
                    &spells,
                    &combination,
                    weapon,
                ) {
                    let url = hppeng_codes.generate_url(
                        Some("9"),
                        Some([
                            combination[2].id,
                            combination[3].id,
                            combination[4].id,
                            combination[5].id,
                            combination[0].id,
                            combination[1].id,
                            combination[6].id,
                            combination[7].id,
                            weapon.id,
                        ]),
                        Some([
                            stat.skill_point.original.e() as i32,
                            stat.skill_point.original.t() as i32,
                            stat.skill_point.original.w() as i32,
                            stat.skill_point.original.f() as i32,
                            stat.skill_point.original.a() as i32,
                        ]),
                        Some(config.player.lvl),
                    );
                    if config.hppeng.log_builds {
                        println!("{}", url);
                        println!("{}", stat);
                        for spell_damage in &stat.spell_damages {
                            println!(
                                "{}: normal {:.2} crit {:.2} avg {:.2}",
                                spell_damage.name,
                                spell_damage.normal,
                                spell_damage.crit,
                                spell_damage.avg
                            )
                        }
                    }

                    let rt = Runtime::new().unwrap();
                    rt.block_on(db::save_build(db_pool.clone(), url, stat, combination));
                };
            }
        },
    );

    println!("done");
}

fn spawn_speed_watcher(counter: Arc<AtomicUsize>, coefficient: usize, combinations: usize) {
    spawn(async move {
        let mut total = 0;
        // Keep track of past 10 speeds and calculate the avg
        let mut last_10_speeds = VecDeque::with_capacity(10);
        loop {
            sleep(Duration::from_secs(1)).await;

            let counter_val = counter.load(Ordering::Acquire);

            let speed = counter_val * coefficient;
            total += speed;

            // Remove 1 from 10 to see if we're nearly at capacity, then pop the last value
            if last_10_speeds.get(10 - 1).is_some() {
                last_10_speeds.pop_back();
            }
            last_10_speeds.push_front(speed);

            let mut remaining_time = usize::MAX;
            if last_10_speeds.front().is_some() {
                let avg_speed = last_10_speeds.iter().sum::<usize>() / last_10_speeds.len();
                if avg_speed > 0 {
                    remaining_time = combinations / avg_speed;
                }
            }

            println!("speed: {}/builds per second", speed);
            println!(
                "remaining time: {:.2}h left",
                remaining_time as f32 / 3600.0
            );
            println!("remaining builds: {}", combinations - total);
            counter.store(0, Ordering::Release);
        }
    });
}

fn load_abilities() -> AbilitiesMap {
    let file = File::open("assets/atree.json")
        .expect("The file `atree.json` should exist in the folder assets.");
    let reader = BufReader::new(file);
    let abilities: AbilitiesMap = serde_json::from_reader(reader).unwrap();
    abilities
}
fn validate_config_damages(spells: &[Spell], config: &Config) -> Result<(), String> {
    let active_abilities: HashSet<String> = spells
        .iter()
        .flat_map(|spell| {
            spell
                .parts
                .iter()
                .map(|part| format!("{}{SPLIT_STR}{}", spell.name, part.name))
        })
        .collect();

    let missing_thresholds: Vec<String> = config
        .threshold_damages
        .iter()
        .filter_map(|threshold| {
            if !active_abilities.contains(&threshold.name) {
                Some(threshold.name.clone())
            } else {
                None
            }
        })
        .collect();

    if !missing_thresholds.is_empty() {
        let mut error = String::from("The following threshold damages are not found:\n");
        missing_thresholds.iter().for_each(|missing| {
            write!(&mut error, "{}\n", missing).unwrap();
        });

        write!(&mut error, "\nActive abilities:\n").unwrap();
        active_abilities.iter().for_each(|ability| {
            write!(&mut error, "{}\n", ability).unwrap();
        });

        return Err(error);
    }

    Ok(())
}

fn retain_spells(spells: &mut Vec<Spell>, config: &Config) {
    let threshold_map: HashMap<&str, HashSet<&str>> = config
        .threshold_damages
        .iter()
        .filter_map(|damage| damage.name.split_once(SPLIT_STR))
        .fold(HashMap::new(), |mut map, (spell, part)| {
            map.entry(spell).or_insert_with(HashSet::new).insert(part);
            map
        });
    spells.retain_mut(|spell| {
        if let Some(parts) = threshold_map.get(spell.name.as_str()) {
            spell
                .parts
                .retain(|part| parts.contains(part.name.as_str()));
            !spell.parts.is_empty()
        } else {
            false
        }
    });
}

fn find<'a>(
    apparels: &'a [Apparel],
    names: &'a [String],
) -> Result<Vec<&'a Apparel>, Vec<&'a String>> {
    let mut results = Vec::with_capacity(names.len());
    let mut errors = Vec::new();

    for name in names {
        match apparels.iter().find(|apparel| &apparel.name == name) {
            Some(apparel) => results.push(apparel),
            None => errors.push(name),
        }
    }

    if errors.is_empty() {
        Ok(results)
    } else {
        Err(errors)
    }
}
pub struct SpellDamage {
    pub name: String,
    pub normal: f64,
    pub crit: f64,
    pub avg: f64,
}
pub struct Status {
    pub max_common_stat: CommonStat,
    pub max_sec_stat: SecStat,
    pub max_hpr: i32,
    pub max_hp: i32,
    pub max_ehp: i32,
    pub max_def: Point,
    pub skill_point: SkillPoints,
    pub max_dam_pct: Dam,
    pub spell_damages: Vec<SpellDamage>,
}
impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "max_common_stat:{}\nmax_sec_stat:{}\nmax_hpr:{}\nmax_hp:{}\nmax_ehp:{}\nskill_point:\n{}\nmax_def:\t{}\nmax_dam_pct:\t{}",
            self.max_common_stat,
            self.max_sec_stat,
            self.max_hpr,
            self.max_hp,
            self.max_ehp,
            self.skill_point,
            self.max_def,
            self.max_dam_pct,
        )
    }
}

fn calculate_spell_damage(
    common_stat: &CommonStat,
    skill_point: &SkillPoints,
    dam_pct: &Dam,
    weapon: &Weapon,
    dam_raw: i32,
    dam_add: &Damages,
    spells: &[Spell],
) -> Vec<(String, f64, f64)> {
    let mut spell_damage = Vec::new();
    for spell in spells {
        for part in &spell.parts {
            let (normal_damage, crit_damage) = damage_calculate(
                &Statistics {
                    ability_dam_convert: Default::default(),
                    dam_convert: Default::default(),
                    skill_point: skill_point.original.clone(),
                    sd_pct: common_stat.sd_pct() as f64 / 100.0,
                    sd_pct_s: Default::default(),
                    dam_pct: Default::default(),
                    dam_pct_s: DamagesConvert::from(dam_pct),
                    r_sd_pct: Default::default(),
                    r_dam_pct: Default::default(),
                    r_sd_raw: Default::default(),
                    r_dam_raw: Default::default(),
                    sd_raw: common_stat.sd_raw() as i32,
                    sd_raw_s: Default::default(),
                    dam_raw,
                    dam_raw_s: Default::default(),
                    crit_dam_pct: Default::default(),
                    dam_add: dam_add.clone(),
                },
                weapon,
                &part.dam_convert,
            );
            spell_damage.push((
                format!("{}{SPLIT_STR}{}", spell.name, part.name),
                normal_damage.total().avg(),
                crit_damage.total().avg(),
            ));
        }
    }
    spell_damage
}
fn calculate_stats(
    config: &Config,
    common_stat: &CommonStat,
    dam_pct: &Dam,
    dam_raw: i32,
    dam_add: &Damages,
    spells: &[Spell],
    combination: &[&Apparel; 8],
    weapon: &Weapon,
) -> Result<Status, String> {
    let max_hp = sum_hp_max(combination, weapon) + config.player.base_hp;
    if let Some(threshold) = &config.hp_threshold() {
        if max_hp < *threshold {
            return Err(String::new());
        }
    }
    let max_common_stat = &CommonStat::sum_max_stats(combination, weapon) + &common_stat;
    if let Some(threshold) = &config.common_stat_threshold() {
        if max_common_stat.any_lt(threshold) {
            return Err(String::new());
        }
    }
    let max_hpr = max_common_stat.hpr();
    if let Some(threshold) = &config.hpr_threshold() {
        if max_hpr < *threshold {
            return Err(String::new());
        }
    }
    let max_def = sum_def_max(combination, weapon);
    if let Some(threshold) = &config.def_threshold() {
        if max_def.any_lt(&threshold) {
            return Err(String::new());
        }
    }
    let max_dam_pct = &sum_dam_pct_max(combination, weapon) + dam_pct;
    if let Some(threshold) = &config.dam_threshold() {
        if max_dam_pct.any_lt(threshold) {
            return Err(String::new());
        }
    }

    if let Some(illegal_combinations) = &config.items.illegal_combinations {
        if is_illegal_combination(combination, illegal_combinations.as_slice()) {
            return Err(String::new());
        }
    }

    if SkillPoints::fast_gap(combination)
        .only_negative()
        .sum()
        .abs()
        > config.player.available_point
    {
        return Err(String::new());
    }
    let (mut skill_point, _) = SkillPoints::scc_put_calculate(combination, weapon);

    if let Some(threshold) = &config.point_threshold() {
        skill_point.assign(threshold);
    }
    if !skill_point.check(config.player.available_point) {
        return Err(String::new());
    }

    let max_ehp = ehp(&skill_point, max_hp, &Class::from(weapon));
    if let Some(threshold) = &config.ehp_threshold() {
        if max_ehp < *threshold {
            return Err(String::new());
        }
    }

    let max_sec_stat = SecStat::sum_max_stats(combination, weapon);
    if let Some(threshold) = &config.sec_stat_threshold() {
        if max_sec_stat.any_lt(threshold) {
            return Err(String::new());
        }
    }

    let crit_pct = skill_points_to_percentage(skill_point.original.t());
    let spell_damages: Vec<SpellDamage> = calculate_spell_damage(
        &max_common_stat,
        &skill_point,
        &max_dam_pct,
        weapon,
        dam_raw,
        dam_add,
        spells,
    )
    .into_iter()
    .map(|(name, normal, crit)| {
        let avg = crit_pct * (crit - normal) + normal;
        SpellDamage {
            name,
            normal,
            crit,
            avg,
        }
    })
    .collect();

    let damage_threshold = config.damage_threshold();
    for v in &spell_damages {
        let (normal_threshold, crit_threshold, avg_threshold) =
            damage_threshold.get(v.name.as_str()).unwrap();
        if (*normal_threshold as f64) > v.normal
            || (*crit_threshold as f64) > v.crit
            || (*avg_threshold as f64) > v.avg
        {
            return Err(String::new());
        }
    }

    Ok(Status {
        max_common_stat,
        max_sec_stat,
        max_hpr,
        max_hp,
        max_def,
        skill_point,
        max_ehp,
        max_dam_pct,
        spell_damages,
    })
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
    false
}
