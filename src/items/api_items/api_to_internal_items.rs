use std::{
    collections::HashMap,
    fs::File,
    hash::{DefaultHasher, Hash, Hasher},
    io::Read,
};

use crate::items::Items;
use crate::Item;

use super::{ApiItems, StatOrInt};

fn string_to_i32_hash(s: &str) -> i32 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish() as i32
}

fn convert_to_uppercase_format(input: &str) -> String {
    input
        .split_whitespace() // Split the string into words
        .map(|word| word.to_uppercase()) // Convert each word to uppercase
        .collect::<Vec<String>>() // Collect the words into a vector
        .join("_") // Join the words with underscores
}

impl From<ApiItems> for Items {
    fn from(api_items: ApiItems) -> Self {
        let mut items = Items { items: Vec::new() };

        for (name, api_item) in api_items {
            let ids = api_item.identifications.as_ref();

            let max_or_int = |option: Option<StatOrInt>| match option {
                Some(v) => match v {
                    StatOrInt::Stat(stat) => Some(stat.max),
                    StatOrInt::Int(int) => Some(int),
                },
                None => None,
            };

            let range_or_int = |option: Option<StatOrInt>| match option {
                Some(v) => match v {
                    StatOrInt::Stat(stat) => Some(format!("{}-{}", stat.min, stat.max)),
                    StatOrInt::Int(int) => Some(format!("{}-{}", int, int)),
                },
                None => None,
            };

            let path = "assets/id_map.json";
            let mut file = File::open(path).expect("fs should be able to open id_map.json file");

            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("fs should be able to read id_map.json file");

            let id_map: HashMap<String, i32> = serde_json::from_str(&contents).unwrap();

            let item_type = if let Some(armour_type) = &api_item.armour_type {
                armour_type.clone().to_string()
            } else if let Some(accessory_type) = &api_item.accessory_type {
                accessory_type.clone().to_string()
            } else if let Some(weapon_type) = &api_item.weapon_type {
                weapon_type.clone().to_string()
            } else {
                "unknown".to_string()
            };

            let item = Item {
                id: match id_map.get(&api_item.internal_name) {
                    Some(id) => *id,
                    None => string_to_i32_hash(&api_item.internal_name),
                },
                name: name.clone(),
                tier: api_item.rarity.to_string(),
                r#type: item_type,
                lvl: api_item.requirements.level,
                fix_id: api_item.identified,
                slots: api_item.powder_slots,
                hp: api_item.base.and_then(|base| base.base_health),
                hp_bonus: ids.and_then(|ids| max_or_int(ids.raw_health)),
                a_def: api_item.base.and_then(|base| base.base_air_defence),
                f_def: api_item.base.and_then(|base| base.base_fire_defence),
                t_def: api_item.base.and_then(|base| base.base_thunder_defence),
                e_def: api_item.base.and_then(|base| base.base_earth_defence),
                w_def: api_item.base.and_then(|base| base.base_water_defence),
                def_req: api_item.requirements.defence,
                str_req: api_item.requirements.strength,
                int_req: api_item.requirements.intelligence,
                agi_req: api_item.requirements.agility,
                dex_req: api_item.requirements.dexterity,
                def: ids.and_then(|ids| ids.raw_defence),
                str: ids.and_then(|ids| ids.raw_strength),
                int: ids.and_then(|ids| ids.raw_intelligence),
                agi: ids.and_then(|ids| ids.raw_agility),
                dex: ids.and_then(|ids| ids.raw_dexterity),
                hpr_raw: ids.and_then(|ids| max_or_int(ids.health_regen_raw)),
                hpr_pct: ids.and_then(|ids| max_or_int(ids.health_regen)),
                a_def_pct: ids.and_then(|ids| max_or_int(ids.air_defence)),
                f_def_pct: ids.and_then(|ids| max_or_int(ids.fire_defence)),
                t_def_pct: ids.and_then(|ids| max_or_int(ids.thunder_defence)),
                e_def_pct: ids.and_then(|ids| max_or_int(ids.earth_defence)),
                w_def_pct: ids.and_then(|ids| max_or_int(ids.water_defence)),
                mr: ids.and_then(|ids| max_or_int(ids.mana_regen)),
                ls: ids.and_then(|ids| max_or_int(ids.life_steal)),
                ms: ids.and_then(|ids| max_or_int(ids.mana_steal)),
                spd: ids.and_then(|ids| max_or_int(ids.walk_speed)),
                sd_raw: ids.and_then(|ids| max_or_int(ids.raw_spell_damage)),
                sd_pct: ids.and_then(|ids| max_or_int(ids.spell_damage)),
                n_dam: api_item
                    .base
                    .and_then(|base| range_or_int(base.base_damage)),
                e_dam: api_item
                    .base
                    .and_then(|base| range_or_int(base.base_earth_damage)),
                t_dam: api_item
                    .base
                    .and_then(|base| range_or_int(base.base_thunder_damage)),
                w_dam: api_item
                    .base
                    .and_then(|base| range_or_int(base.base_water_damage)),
                f_dam: api_item
                    .base
                    .and_then(|base| range_or_int(base.base_fire_damage)),
                a_dam: api_item
                    .base
                    .and_then(|base| range_or_int(base.base_air_damage)),
                atk_spd: api_item
                    .attack_speed
                    .map(|atk_spd| convert_to_uppercase_format(&atk_spd.to_string())),
                n_dam_pct: ids.and_then(|ids| max_or_int(ids.neutral_damage)),
                e_dam_pct: ids.and_then(|ids| max_or_int(ids.earth_damage)),
                t_dam_pct: ids.and_then(|ids| max_or_int(ids.thunder_damage)),
                w_dam_pct: ids.and_then(|ids| max_or_int(ids.water_damage)),
                f_dam_pct: ids.and_then(|ids| max_or_int(ids.fire_damage)),
                a_dam_pct: ids.and_then(|ids| max_or_int(ids.air_damage)),
                xpb: ids.and_then(|ids| max_or_int(ids.xp_bonus)),
            };

            items.items.push(item);
        }

        items
    }
}
