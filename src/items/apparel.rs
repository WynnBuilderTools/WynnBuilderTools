use crate::calculate::*;

use super::*;

#[derive(Clone, Debug, Default)]
pub struct Apparel {
    pub id: i32,
    pub name: String,
    pub tier: String,
    pub r#type: String,
    pub lvl: i32,
    pub slots: i32,
    pub hp: i32,
    pub hp_bonus_max: i32,
    pub hp_bonus_min: i32,
    pub req: Point,
    pub add: Point,
    pub def: Point,

    pub def_pct_max: Point,
    pub def_pct_min: Point,
    pub dam_pct_max: Dam,
    pub dam_pct_min: Dam,

    pub common_stat_max: CommonStat,
    pub common_stat_min: CommonStat,

    pub sec_stat_max: SecStat,
    pub sec_stat_min: SecStat,

    pub fix_id: bool,
}

impl AsRef<Apparel> for Apparel {
    fn as_ref(&self) -> &Apparel {
        self
    }
}

impl TryFrom<&Item> for Apparel {
    type Error = String;

    fn try_from(value: &Item) -> Result<Self, Self::Error> {
        let req = value.as_req();
        let add = value.as_add();
        let common_stat = value.as_common_stat();
        let sec_stat = value.as_sec_stat();
        let def_pct = value.as_def_pct();
        let def = value.as_def();
        let fix_id = value.as_fix_id();
        let dam_pct = value.as_dam_pct();

        Ok(Apparel {
            id: value.id,
            name: value.name.clone(),
            tier: value.tier.clone(),
            r#type: value.r#type.clone(),
            lvl: value.lvl,
            slots: value.slots.unwrap_or(0),
            hp: value.hp.unwrap_or(0),
            hp_bonus_max: max_roll(&value.hp_bonus.unwrap_or(0), fix_id),
            hp_bonus_min: min_roll(&value.hp_bonus.unwrap_or(0), fix_id),
            def,
            req,
            add,
            def_pct_max: max_roll(&def_pct, fix_id),
            def_pct_min: min_roll(&def_pct, fix_id),
            dam_pct_max: max_roll(&dam_pct, fix_id),
            dam_pct_min: min_roll(&dam_pct, fix_id),
            common_stat_max: max_roll(&common_stat, fix_id),
            common_stat_min: min_roll(&common_stat, fix_id),
            sec_stat_max: max_roll(&sec_stat, fix_id),
            sec_stat_min: min_roll(&sec_stat, fix_id),
            fix_id,
        })
    }
}
