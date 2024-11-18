use std::str::FromStr;

use crate::*;

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
    pub damage: [Range; 6],
    pub atk_spd: AtkSpd,

    pub req: Point,
    pub add: Point,
    pub stat_max: CommonStat,
    pub stat_min: CommonStat,
    pub def_pct_max: Point,
    pub def_pct_min: Point,
    pub dam_pct_max: Dam,
    pub dam_pct_min: Dam,

    pub min_exp_bonus: i32,
    pub max_exp_bonus: i32,

    pub min_loot_bonus: i32,
    pub max_loot_bonus: i32,

    pub fix_id: bool,
}

impl TryFrom<&Item> for Weapon {
    type Error = String;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let req = value.as_req();
        let add = value.as_add();
        let stat = value.as_comm_stat();
        let def_pct = value.as_def_pct();
        let dam_pct = value.as_dam_pct();
        let exp_bonus = value.xpb.unwrap_or(0);
        let loot_bonus = value.loot_bonus.unwrap_or(0);
        let fix_id = value.as_fix_id();

        Ok(Self {
            id: value.id,
            name: value.name.clone(),
            req,
            add,
            fix_id,
            hp_bonus_max: max_roll(&value.hp_bonus.unwrap_or(0), fix_id),
            hp_bonus_min: min_roll(&value.hp_bonus.unwrap_or(0), fix_id),
            stat_max: max_roll(&stat, false),
            stat_min: min_roll(&stat, false),
            def_pct_max: max_roll(&def_pct, fix_id),
            def_pct_min: min_roll(&def_pct, fix_id),
            dam_pct_max: max_roll(&dam_pct, fix_id),
            dam_pct_min: min_roll(&dam_pct, fix_id),
            damage: [
                Range::from_str(value.n_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.e_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.t_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.w_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.f_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
                Range::from_str(value.a_dam.as_ref().map_or("0-0", |v| v.as_str()))?,
            ],
            r#type: WeaponTypes::from_str(value.r#type.as_str())?,
            atk_spd: AtkSpd::from_str(
                value
                    .atk_spd
                    .as_ref()
                    .ok_or("atkSpd is missing".to_string())?
                    .as_str(),
            )?,
            min_exp_bonus: min_roll(&exp_bonus, fix_id),
            max_exp_bonus: max_roll(&exp_bonus, fix_id),
            min_loot_bonus: min_roll(&loot_bonus, fix_id),
            max_loot_bonus: max_roll(&loot_bonus, fix_id),
        })
    }
}
