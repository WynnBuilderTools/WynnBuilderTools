use super::*;

#[derive(Clone, Debug, Default)]
pub struct Apparel {
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

impl TryFrom<&WApiItem> for Apparel {
    type Error = String;

    fn try_from(value: &WApiItem) -> Result<Self, Self::Error> {
        Ok(Apparel {
            name: value.internal_name.clone(),
            tier: value.rarity.clone().unwrap().to_string(),
            r#type: value.item_type().unwrap(),
            lvl: value.requirements.level,
            slots: value.powder_slots.unwrap_or(0),
            hp: value.base.and_then(|base| base.base_health).unwrap_or(0),
            hp_bonus_max: value.hp_bonus_max(),
            hp_bonus_min: value.hp_bonus_min(),
            req: value.req(),
            add: value.add().unwrap(),
            def: value.def(),
            def_pct_max: value.def_pct_max(),
            def_pct_min: value.def_pct_min(),
            dam_pct_max: value.dam_pct_max(),
            dam_pct_min: value.dam_pct_min(),
            common_stat_max: value.common_stat_max(),
            common_stat_min: value.common_stat_min(),
            sec_stat_max: value.sec_stat_max(),
            sec_stat_min: value.sec_stat_min(),
            fix_id: value.identified.unwrap_or(false),
        })
    }
}
