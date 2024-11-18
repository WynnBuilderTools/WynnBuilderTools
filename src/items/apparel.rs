use crate::*;

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

    pub min_exp_bonus: i32,
    pub max_exp_bonus: i32,

    pub min_loot_bonus: i32,
    pub max_loot_bonus: i32,

    pub stat_max: CommonStat,
    pub stat_min: CommonStat,

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
        let stat = value.as_comm_stat();
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
            min_exp_bonus: min_roll(&value.xpb.unwrap_or(0), fix_id),
            max_exp_bonus: max_roll(&value.xpb.unwrap_or(0), fix_id),
            min_loot_bonus: min_roll(&value.loot_bonus.unwrap_or(0), fix_id),
            max_loot_bonus: max_roll(&value.loot_bonus.unwrap_or(0), fix_id),
            stat_max: max_roll(&stat, fix_id),
            stat_min: min_roll(&stat, fix_id),
            fix_id,
        })
    }
}
