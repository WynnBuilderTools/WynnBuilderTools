use std::hash::{DefaultHasher, Hash, Hasher};

use crate::items::api_items::api_item::ApiItem;
use crate::items::Items as InternalItems;
use crate::Item as InternalItem;

use super::{ApiItems, Stat, StatOrInt};

fn string_to_i32_hash(s: &str) -> i32 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish() as i32
}

impl From<ApiItems> for InternalItems {
    fn from(api_items: ApiItems) -> Self {
        let mut items = InternalItems { items: Vec::new() };

        for (_, api_item) in api_items.items {
            items.items.push(api_item.into());
        }

        items
    }
}

impl From<ApiItem> for InternalItem {
    fn from(api_item: ApiItem) -> Self {
        let ids = api_item.identifications.as_ref();

        let ident = |stat: Option<Stat>| stat.map(|stat| stat.max());
        let max_or_int = |stat: StatOrInt| {
            match stat {
                StatOrInt::Stat(stat) => stat.max(),
                StatOrInt::Int(i) => i,
            }
        };

        InternalItem {
            id: string_to_i32_hash(&api_item.internal_name),
            name: api_item.internal_name,
            tier: api_item.rarity.to_string(),
            r#type: api_item.type_field.to_string(),
            lvl: api_item.requirements.level,
            fix_id: api_item.identified,
            slots: api_item.powder_slots,
            hp: api_item
                .base
                .and_then(|base| base.base_health.map(|stat| stat)),
            hp_bonus: ids.and_then(|ids| {
                ids.raw_health.as_ref().map(|stat| match stat {
                    super::StatOrInt::Stat(stat) => stat.max(),
                    super::StatOrInt::Int(int) => *int,
                })
            }),
            a_def: api_item
                .base
                .as_ref()
                .and_then(|base| base.base_air_defence),
            f_def: api_item
                .base
                .as_ref()
                .and_then(|base| base.base_fire_defence),
            t_def: api_item
                .base
                .as_ref()
                .and_then(|base| base.base_thunder_defence),
            e_def: api_item
                .base
                .as_ref()
                .and_then(|base| base.base_earth_defence),
            w_def: api_item
                .base
                .as_ref()
                .and_then(|base| base.base_water_defence),
            def_req: api_item.requirements.defence,
            str_req: api_item.requirements.strength,
            int_req: api_item.requirements.intelligence,
            agi_req: api_item.requirements.agility,
            dex_req: api_item.requirements.dexterity,
            def: ids.and_then(|ids| ids.raw_defence),
            str: ids.and_then(|ids| ids.raw_strength.clone()),
            int: ids.and_then(|ids| ids.raw_intelligence.clone()),
            agi: ids.and_then(|ids| ids.raw_agility.clone()),
            dex: ids.and_then(|ids| ids.raw_dexterity.clone()),
            hpr_raw: ids.and_then(|ids| ids.health_regen_raw.and_then(|stat| Some(max_or_int(stat)))),
            hpr_pct: ids.and_then(|ids| ids.health_regen.and_then(|stat| Some(max_or_int(stat)))),
            a_def_pct: ids.and_then(|ids| ids.air_defence.and_then(|stat| Some(max_or_int(stat)))),
            f_def_pct: ids.and_then(|ids| ids.fire_defence.and_then(|stat| Some(max_or_int(stat)))),
            t_def_pct: ids.and_then(|ids| ids.thunder_defence.and_then(|stat| Some(max_or_int(stat)))),
            e_def_pct: ids.and_then(|ids| ids.earth_defence.and_then(|stat| Some(max_or_int(stat)))),
            w_def_pct: ids.and_then(|ids| ids.water_defence.and_then(|stat| Some(max_or_int(stat)))),
            mr: ids.and_then(|ids| ids.mana_regen.and_then(|stat| Some(max_or_int(stat)))),
            ls: ids.and_then(|ids| ids.life_steal.and_then(|stat| Some(max_or_int(stat)))),
            ms: ids.and_then(|ids| ids.mana_steal.and_then(|stat| Some(max_or_int(stat)))),
            spd: ids.and_then(|ids| ids.walk_speed.and_then(|stat| Some(max_or_int(stat)))),
            sd_raw: ids.and_then(|ids| ids.raw_spell_damage.and_then(|stat| Some(max_or_int(stat)))),
            sd_pct: ids.and_then(|ids| ids.spell_damage.and_then(|stat| Some(max_or_int(stat)))),
            n_dam: ids.and_then(|ids| ids.damage.and_then(|stat| Some(max_or_int(stat).to_string()))),
            e_dam: ids.and_then(|ids| ids.earth_damage.and_then(|stat| Some(max_or_int(stat).to_string()))),
            t_dam: ids.and_then(|ids| ids.thunder_damage.and_then(|stat| Some(max_or_int(stat).to_string()))),
            w_dam: ids.and_then(|ids| ids.water_damage.and_then(|stat| Some(max_or_int(stat).to_string()))),
            f_dam: ids.and_then(|ids| ids.fire_damage.and_then(|stat| Some(max_or_int(stat).to_string()))),
            a_dam: ids.and_then(|ids| ids.air_damage.and_then(|stat| Some(max_or_int(stat).to_string()))),
            atk_spd: ids.and_then(|ids| ids.raw_attack_speed.and_then(|stat| Some(max_or_int(stat).to_string()))),
            n_dam_pct: ids.and_then(|ids| ids.health_regen_raw.and_then(|stat| Some(max_or_int(stat)))),
            e_dam_pct: ids.and_then(|ids| ids.health_regen_raw.and_then(|stat| Some(max_or_int(stat)))),
            t_dam_pct: ids.and_then(|ids| ids.health_regen_raw.and_then(|stat| Some(max_or_int(stat)))),
            w_dam_pct: ids.and_then(|ids| ids.health_regen_raw.and_then(|stat| Some(max_or_int(stat)))),
            f_dam_pct: ids.and_then(|ids| ids.health_regen_raw.and_then(|stat| Some(max_or_int(stat)))),
            a_dam_pct: ids.and_then(|ids| ids.health_regen_raw.and_then(|stat| Some(max_or_int(stat)))),
            xpb: ids.and_then(|ids| ids.health_regen_raw.and_then(|stat| Some(max_or_int(stat)))),
        }
    }
}
