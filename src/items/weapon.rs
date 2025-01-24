use std::str::FromStr;

use super::*;

#[derive(Clone, Debug, Default)]
pub struct Weapon {
    pub name: String,
    pub r#type: WeaponTypes,

    pub hp_bonus_max: i32,
    pub hp_bonus_min: i32,
    // "nDam_"
    // "eDam_"
    // "tDam_"
    // "wDam_"
    // "fDam_"
    // "aDam_"
    pub damage: Damages,
    pub atk_spd: AtkSpd,

    pub req: Point,
    pub add: Point,
    pub common_stat_max: CommonStat,
    pub common_stat_min: CommonStat,
    pub sec_stat_max: SecStat,
    pub sec_stat_min: SecStat,
    pub def_pct_max: Point,
    pub def_pct_min: Point,
    pub dam_pct_max: Dam,
    pub dam_pct_min: Dam,

    pub damage_present: Mask,

    pub fix_id: bool,
}

impl TryFrom<&WApiItem> for Weapon {
    type Error = String;

    fn try_from(value: &WApiItem) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.internal_name.clone(),
            r#type: WeaponTypes::from_str(&value.item_type().unwrap())?,
            hp_bonus_max: value.hp_bonus_max(),
            hp_bonus_min: value.hp_bonus_min(),
            damage: value.damages(),
            atk_spd: value.attack_speed().unwrap(),
            req: value.req(),
            add: value.add()?,
            common_stat_max: value.common_stat_max(),
            common_stat_min: value.common_stat_min(),
            sec_stat_max: value.sec_stat_max(),
            sec_stat_min: value.sec_stat_min(),
            def_pct_max: value.def_pct_max(),
            def_pct_min: value.def_pct_min(),
            dam_pct_max: value.dam_pct_max(),
            dam_pct_min: value.dam_pct_min(),
            damage_present: value.damage_present(),
            fix_id: value.identified.unwrap_or(false),
        })
    }
}
