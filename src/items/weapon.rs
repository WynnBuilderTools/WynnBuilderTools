use std::str::FromStr;

use crate::calculate::*;

use super::*;

#[derive(Clone, Debug, Default)]
pub struct Weapon {
    pub id: i32,
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

impl TryFrom<&Item> for Weapon {
    type Error = String;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let req = value.as_req();
        let add = value.as_add();
        let common_stat = value.as_common_stat();
        let sec_stat = value.as_sec_stat();
        let def_pct = value.as_def_pct();
        let dam_pct = value.as_dam_pct();
        let fix_id = value.as_fix_id();

        Ok(Self {
            id: value.id,
            name: value.name.clone(),
            req,
            add,
            fix_id,
            hp_bonus_max: max_roll(&value.hp_bonus.unwrap_or(0), fix_id),
            hp_bonus_min: min_roll(&value.hp_bonus.unwrap_or(0), fix_id),
            common_stat_max: max_roll(&common_stat, false),
            common_stat_min: min_roll(&common_stat, false),
            sec_stat_max: max_roll(&sec_stat, false),
            sec_stat_min: min_roll(&sec_stat, false),
            def_pct_max: max_roll(&def_pct, fix_id),
            def_pct_min: min_roll(&def_pct, fix_id),
            dam_pct_max: max_roll(&dam_pct, fix_id),
            dam_pct_min: min_roll(&dam_pct, fix_id),
            damage: Damages::from_slice([
                Range::from_str(value.n_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.e_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.t_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.w_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.f_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.a_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
            ]),
            damage_present: Mask::from_slice([
                value.n_dam.is_some(),
                value.e_dam.is_some(),
                value.t_dam.is_some(),
                value.w_dam.is_some(),
                value.f_dam.is_some(),
                value.a_dam.is_some(),
            ]),
            r#type: WeaponTypes::from_str(value.r#type.as_str())?,
            atk_spd: AtkSpd::from_str(
                value
                    .atk_spd
                    .as_ref()
                    .ok_or("atkSpd is missing".to_string())?
                    .as_str(),
            )?,
        })
    }
}
