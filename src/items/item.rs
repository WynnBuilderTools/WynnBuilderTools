use serde::Deserialize;

use crate::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(serde::Serialize)]
pub struct Items {
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(serde::Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub tier: String,
    pub r#type: String,
    pub lvl: i32,
    #[serde(rename = "fixID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_id: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slots: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hp_bonus: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_def: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_def: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_def: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_def: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w_def: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def_req: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_req: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int_req: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agi_req: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex_req: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub int: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agi: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dex: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpr_raw: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hpr_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_def_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_def_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_def_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_def_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w_def_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mr: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ls: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spd: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sd_raw: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sd_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_dam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_dam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_dam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w_dam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_dam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_dam: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atk_spd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_dam_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_dam_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_dam_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub w_dam_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_dam_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_dam_pct: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpb: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_bonus: Option<i32>,
}
impl Item {
    pub fn as_req(&self) -> Point {
        Point::new(
            self.str_req.map_or(0, |v| v as i16),
            self.dex_req.map_or(0, |v| v as i16),
            self.int_req.map_or(0, |v| v as i16),
            self.def_req.map_or(0, |v| v as i16),
            self.agi_req.map_or(0, |v| v as i16),
        )
    }
    pub fn as_add(&self) -> Point {
        Point::new(
            self.str.map_or(0, |v| v as i16),
            self.dex.map_or(0, |v| v as i16),
            self.int.map_or(0, |v| v as i16),
            self.def.map_or(0, |v| v as i16),
            self.agi.map_or(0, |v| v as i16),
        )
    }
    pub fn as_common_stat(&self) -> CommonStat {
        CommonStat::new(
            self.hpr_raw.map_or(0, |v| v as i16),
            self.hpr_pct.map_or(0, |v| v as i16),
            self.mr.map_or(0, |v| v as i16),
            self.ls.map_or(0, |v| v as i16),
            self.ms.map_or(0, |v| v as i16),
            self.spd.map_or(0, |v| v as i16),
            self.sd_raw.map_or(0, |v| v as i16),
            self.sd_pct.map_or(0, |v| v as i16),
        )
    }
    pub fn as_sec_stat(&self) -> SecStat {
        SecStat::new(
            self.loot_bonus.map_or(0, |v| v as i16),
            self.xpb.map_or(0, |v| v as i16),
        )
    }
    pub fn as_def(&self) -> Point {
        Point::new(
            self.e_def.map_or(0, |v| v as i16),
            self.t_def.map_or(0, |v| v as i16),
            self.w_def.map_or(0, |v| v as i16),
            self.f_def.map_or(0, |v| v as i16),
            self.a_def.map_or(0, |v| v as i16),
        )
    }
    pub fn as_def_pct(&self) -> Point {
        Point::new(
            self.e_def_pct.map_or(0, |v| v as i16),
            self.t_def_pct.map_or(0, |v| v as i16),
            self.w_def_pct.map_or(0, |v| v as i16),
            self.f_def_pct.map_or(0, |v| v as i16),
            self.a_def_pct.map_or(0, |v| v as i16),
        )
    }
    pub fn as_dam_pct(&self) -> Dam {
        Dam::new(
            self.n_dam_pct.map_or(0, |v| v as i16),
            self.e_dam_pct.map_or(0, |v| v as i16),
            self.t_dam_pct.map_or(0, |v| v as i16),
            self.w_dam_pct.map_or(0, |v| v as i16),
            self.f_dam_pct.map_or(0, |v| v as i16),
            self.a_dam_pct.map_or(0, |v| v as i16),
        )
    }
    pub fn as_fix_id(&self) -> bool {
        self.fix_id.map_or(false, |v| v)
    }
}
pub fn max_roll<T: Roll>(value: &T, fix_id: bool) -> T {
    value.max_roll(fix_id)
}
pub fn min_roll<T: Roll>(value: &T, fix_id: bool) -> T {
    value.min_roll(fix_id)
}

pub trait Roll {
    fn min_roll(&self, fix_id: bool) -> Self;
    fn max_roll(&self, fix_id: bool) -> Self;
}

impl Roll for i32 {
    fn min_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            *self
        } else {
            min_rolls_i32(*self)
        }
    }

    fn max_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            *self
        } else {
            max_rolls_i32(*self)
        }
    }
}

impl Roll for Point {
    fn min_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: max_rolls(&self.inner),
            }
        }
    }

    fn max_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: max_rolls(&self.inner),
            }
        }
    }
}
impl Roll for Dam {
    fn min_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: min_rolls(&self.inner),
            }
        }
    }

    fn max_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: max_rolls(&self.inner),
            }
        }
    }
}
impl Roll for CommonStat {
    fn min_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: min_rolls(&self.inner),
            }
        }
    }

    fn max_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: max_rolls(&self.inner),
            }
        }
    }
}
impl Roll for SecStat {
    fn min_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: min_rolls(&self.inner),
            }
        }
    }

    fn max_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: max_rolls(&self.inner),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn derivative_works() {
        let file = File::open("config/items.json")
            .expect("The file `items.json` should exist in the folder config.");
        let reader = BufReader::new(file);

        let _: Items = serde_json::from_reader(reader).unwrap();
    }
}

// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/HEAD/js/build_utils.js#L136
//let rolledIDs = [
//"hprPct",
//"mr",
//"sdPct",
//"mdPct",
//"ls",
//"ms",
//"xpb",
//"lb",
//"ref",
//"thorns",
//"expd",
//"spd",
//"atkTier",
//"poison",
//"hpBonus",
//"spRegen",
//"eSteal",
//"hprRaw",
//"sdRaw",
//"mdRaw",
//"fDamPct", "wDamPct", "aDamPct", "tDamPct", "eDamPct",
//"fDefPct", "wDefPct", "aDefPct", "tDefPct", "eDefPct",
//"spPct1", "spRaw1",
//"spPct2", "spRaw2",
//"spPct3", "spRaw3",
//"spPct4", "spRaw4",
//"rSdRaw",
//"sprint",
//"sprintReg",
//"jh",
//"lq",
//"gXp",
//"gSpd",
//// wynn2 damages.
//"eMdPct","eMdRaw","eSdPct","eSdRaw","eDamRaw","eDamAddMin","eDamAddMax",
//"tMdPct","tMdRaw","tSdPct","tSdRaw","tDamRaw","tDamAddMin","tDamAddMax",
//"wMdPct","wMdRaw","wSdPct","wSdRaw","wDamRaw","wDamAddMin","wDamAddMax",
//"fMdPct","fMdRaw","fSdPct","fSdRaw","fDamRaw","fDamAddMin","fDamAddMax",
//"aMdPct","aMdRaw","aSdPct","aSdRaw","aDamRaw","aDamAddMin","aDamAddMax",
//"nMdPct","nMdRaw","nSdPct","nSdRaw","nDamPct","nDamRaw","nDamAddMin","nDamAddMax",    // neutral which is now an element
//"damPct","damRaw","damAddMin","damAddMax",                                            // These are the old ids. Become proportional.
//"rMdPct","rMdRaw","rSdPct","rDamPct","rDamRaw","rDamAddMin","rDamAddMax",             // rainbow (the "element" of all minus neutral). rSdRaw is rainraw
//"spPct1Final", "spPct2Final", "spPct3Final", "spPct4Final"
//];
