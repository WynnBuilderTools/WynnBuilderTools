use std::hash::{DefaultHasher, Hash, Hasher};

use crate::items::api_items::api_item::ApiItem;
use crate::items::Items as InternalItems;
use crate::Item as InternalItem;

use super::{ApiItems, IdentificationStat};

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
        };

        items
    }
}

impl From<ApiItem> for InternalItem {
    fn from(api_item: ApiItem) -> Self {
        let identifications = api_item.identifications.as_ref();

        let ident = |stat: Option<IdentificationStat>| {
            stat.map(|stat| stat.max())
        };

        InternalItem {
            id: string_to_i32_hash(api_item.internal_name.as_ref().unwrap_or(&"Unnamed".to_string())),
            name: api_item.internal_name.unwrap_or("Unnamed".to_string()),
            tier: api_item.rarity.clone(),
            r#type: api_item.type_field.clone(),
            lvl: api_item.requirements.level,
            fix_id: api_item.identified,
            slots: api_item.powder_slots,
            hp: api_item.base.base_health,
            hp_bonus: identifications.and_then(|ids| ids.raw_health.map(|stat| stat.max())),
            a_def: api_item.base.base_air_defence,
            f_def: api_item.base.base_fire_defence,
            t_def: api_item.base.base_thunder_defence,
            e_def: api_item.base.base_earth_defence,
            w_def: api_item.base.base_water_defence,
            def_req: api_item.requirements.defence,
            str_req: api_item.requirements.strength,
            int_req: api_item.requirements.intelligence,
            agi_req: api_item.requirements.agility,
            dex_req: api_item.requirements.dexterity,
            def: identifications.and_then(|ids| ids.raw_defence),
            str: identifications.and_then(|ids| ids.raw_strength.clone()),
            int: identifications.and_then(|ids| ids.raw_intelligence.clone()),
            agi: identifications.and_then(|ids| ids.raw_agility.clone()),
            dex: identifications.and_then(|ids| ids.raw_dexterity.clone()),
            hpr_raw: identifications.and_then(|ids| ids.raw_health_regen.map(|stat| stat.max())),
            hpr_pct: ident(identifications.and_then(|ids| ids.health_regen_pct)),
            a_def_pct: ident(identifications.and_then(|ids| ids.air_defence)),
            f_def_pct: ident(identifications.and_then(|ids| ids.fire_defence)),
            t_def_pct: ident(identifications.and_then(|ids| ids.thunder_defence)),
            e_def_pct: ident(identifications.and_then(|ids| ids.earth_defence)),
            w_def_pct: ident(identifications.and_then(|ids| ids.water_defence)),
            mr: ident(identifications.and_then(|ids| ids.mana_regen)),
            ls: ident(identifications.and_then(|ids| ids.life_steal)),
            ms: ident(identifications.and_then(|ids| ids.mana_steal)),
            spd: ident(identifications.and_then(|ids| ids.walk_speed)),
            sd_raw: ident(identifications.and_then(|ids| ids.raw_spell_damage)),
            sd_pct: ident(identifications.and_then(|ids| ids.spell_damage)),
            n_dam: api_item.base.base_damage.map(|dmg| dmg.max.to_string()),
            e_dam: api_item
                .base
                .base_earth_damage
                .map(|dmg| dmg.max.to_string()),
            t_dam: api_item
                .base
                .base_thunder_damage
                .map(|dmg| dmg.max.to_string()),
            w_dam: api_item
                .base
                .base_water_damage
                .map(|dmg| dmg.max.to_string()),
            f_dam: api_item
                .base
                .base_fire_damage
                .map(|dmg| dmg.max.to_string()),
            a_dam: api_item.base.base_air_damage.map(|dmg| dmg.max.to_string()),
            atk_spd: api_item.attack_speed.clone(),
            n_dam_pct: ident(identifications.and_then(|ids| ids.neutral_damage)),
            e_dam_pct: ident(identifications.and_then(|ids| ids.earth_damage)),
            t_dam_pct: ident(identifications.and_then(|ids| ids.thunder_damage)),
            w_dam_pct: ident(identifications.and_then(|ids| ids.water_damage)),
            f_dam_pct: ident(identifications.and_then(|ids| ids.fire_damage)),
            a_dam_pct: ident(identifications.and_then(|ids| ids.air_damage)),
            xpb: ident(identifications.and_then(|ids| ids.exp_bonus)),
        }
    }
}
