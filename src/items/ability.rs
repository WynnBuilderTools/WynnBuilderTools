use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use super::*;

pub type AbilitiesMap = HashMap<Class, Vec<ATreeNodeData>>;

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L36
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Effect {
    ReplaceSpell(ReplaceSpell),
    AddSpellProp(AddSpellProp),
    ConvertSpellConv(ConvertSpellConv),
    /// fixed value ability effect
    RawStat(RawStat),
    /// dynamically ability effect
    StatScaling(StatScaling),
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L15
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ATreeNodeData {
    pub display_name: String,
    pub id: i32,
    pub parents: Vec<i32>,
    pub base_abil: Option<i32>,
    pub cost: i32,
    pub effects: Vec<Effect>,
}

impl ATreeNodeData {
    pub fn join_spell(&self) -> HashMap<i32, Spell> {
        let mut spell: HashMap<i32, Spell> = HashMap::new();
        // name base_id damage cost
        for effect in &self.effects {
            match effect {
                Effect::ReplaceSpell(replace_spell) => {
                    if let Some(spell) = spell.get(&replace_spell.base_spell) {
                        println!("{:?}\n{:?}", spell, replace_spell)
                    }
                    spell.insert(
                        replace_spell.base_spell,
                        Spell::new(
                            replace_spell.name.clone(),
                            replace_spell.base_spell,
                            replace_spell.cost.unwrap_or(0),
                            replace_spell
                                .parts
                                .iter()
                                .filter_map(|part| match part {
                                    Part::Damage { name, multipliers } => Some(DamagePart::new(
                                        name.clone(),
                                        DamagesConvert::from_slice_i32(multipliers),
                                    )),
                                    Part::Total { name: _, hits: _ } => None,
                                    Part::Heal { name: _, power: _ } => None,
                                })
                                .collect(),
                        ),
                    );
                }
                _ => continue,
            }
        }
        spell
    }
    /// return Map<id,(cost,parts)>
    pub fn join_spell_add(&self) -> HashMap<i32, (i32, Vec<DamagePart>)> {
        let mut part_add: HashMap<i32, (i32, Vec<DamagePart>)> = HashMap::new();

        for effect in &self.effects {
            if let Effect::AddSpellProp(add_spell_prop) = effect {
                match add_spell_prop.behavior {
                    Behavior::Merge => (),
                    // "modify" not work?
                    // https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/a30a0aa1a8b9f857ae66d31e67bdc5df0a4d0cfc/js/builder/atree.js#L941
                    Behavior::Modify => continue,
                    Behavior::Overwrite => continue,
                }

                let (cost, parts) = part_add
                    .entry(add_spell_prop.base_spell)
                    .or_insert_with(|| (0, Vec::new()));

                // https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/a30a0aa1a8b9f857ae66d31e67bdc5df0a4d0cfc/js/builder/atree.js#L927
                *cost += add_spell_prop.cost.unwrap_or(0);

                if let (Some(name), Some(multipliers)) =
                    (&add_spell_prop.target_part, add_spell_prop.multipliers)
                {
                    parts.push(DamagePart::new(
                        name.clone(),
                        DamagesConvert::from_slice_i32(&multipliers),
                    ));
                }
            }
        }

        part_add
    }
    /// return (common_stat, dam_raw, dam_pct, dam_add)
    pub fn join_stat(&self) -> (CommonStat, i32, Dam, Damages) {
        let mut common_stat: CommonStat = Default::default();
        let mut dam_raw: i32 = 0;
        let mut dam_pct: Dam = Default::default();
        let mut dam_add: Damages = Default::default();
        for effect in &self.effects {
            match effect {
                Effect::RawStat(raw_stat) => match raw_stat.toggle {
                    // optional effects are not currently supported
                    Some(_) => continue,
                    None => match raw_stat.behavior {
                        // currently there(atree.json) is no "modify" type in raw_stat
                        Behavior::Modify => continue,
                        Behavior::Overwrite => continue,
                        Behavior::Merge => {
                            // TODO: calculate bonuses during initialization
                            for bonus in &raw_stat.bonuses {
                                match bonus {
                                    // Prop not relevant to damage calculation
                                    StatBonus::Prop {
                                        abil: _,
                                        name: _,
                                        value: _,
                                    } => continue,
                                    StatBonus::Stat { name, value } => match name {
                                        StatName::DamMult(_) => continue,
                                        StatName::DefMult(_) => continue,
                                        StatName::HealMult(_) => continue,
                                        // TODO: support jump height
                                        StatName::JH => continue,
                                        StatName::SPD => {
                                            common_stat +=
                                                &CommonStat::new(0, 0, 0, 0, 0, *value as i16, 0, 0)
                                        }
                                        StatName::DamRaw => dam_raw += value,
                                        // this is only used by Shaman
                                        // handle it later
                                        StatName::SpPct1Final => continue,
                                        StatName::SpPct2Final => continue,
                                        StatName::SpPct3Final => continue,
                                        StatName::EDamPct => {
                                            dam_pct += &Dam::new(0, *value as i16, 0, 0, 0, 0)
                                        }
                                        StatName::TDamPct => {
                                            dam_pct += &Dam::new(0, 0, *value as i16, 0, 0, 0)
                                        }
                                        StatName::WDamPct => {
                                            dam_pct += &Dam::new(0, 0, 0, *value as i16, 0, 0)
                                        }
                                        StatName::FDamPct => {
                                            dam_pct += &Dam::new(0, 0, 0, 0, *value as i16, 0)
                                        }
                                        StatName::ADamPct => {
                                            dam_pct += &Dam::new(0, 0, 0, 0, 0, *value as i16)
                                        }
                                        StatName::EDamAddMax => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                *value as f64,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                            )
                                        }
                                        StatName::TDamAddMax => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                *value as f64,
                                                0.0,
                                                0.0,
                                                0.0,
                                            )
                                        }
                                        StatName::WDamAddMax => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                *value as f64,
                                                0.0,
                                                0.0,
                                            )
                                        }
                                        StatName::FDamAddMax => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                *value as f64,
                                                0.0,
                                            )
                                        }
                                        StatName::ADamAddMax => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                *value as f64,
                                            )
                                        }
                                        StatName::EDamAddMin => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                *value as f64,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                            )
                                        }
                                        StatName::TDamAddMin => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                0.0,
                                                *value as f64,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                            )
                                        }
                                        StatName::WDamAddMin => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                0.0,
                                                0.0,
                                                *value as f64,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                            )
                                        }
                                        StatName::FDamAddMin => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                *value as f64,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                            )
                                        }
                                        StatName::ADamAddMin => {
                                            dam_add += &Damages::new(
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                *value as f64,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                                0.0,
                                            )
                                        }
                                    },
                                }
                            }
                        }
                    },
                },
                _ => continue,
            }
        }
        (common_stat, dam_raw, dam_pct, dam_add)
    }
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/damage_calc.js#L233
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplaceSpell {
    name: String,
    cost: Option<i32>,
    base_spell: i32,
    spell_type: Option<String>,
    scaling: Option<String>,
    // only 4 ability use this attribute
    use_atkspd: Option<bool>,
    parts: Vec<Part>,
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/damage_calc.js#L252
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Part {
    Damage {
        name: String,
        multipliers: [i32; 6],
    },
    Heal {
        name: String,
        power: f64,
    },
    Total {
        name: String,
        hits: HashMap<String, NumberOrString>,
    },
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L43
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddSpellProp {
    base_spell: i32,
    target_part: Option<String>,
    #[serde(default)]
    behavior: Behavior,
    cost: Option<i32>,
    multipliers: Option<[i32; 6]>,
    /// only heal use power
    power: Option<f32>,
    hits: Option<HashMap<String, NumberOrString>>,
    display: Option<String>,
    hide: Option<bool>,
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L63
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConvertSpellConv {
    pub base_spell: i32,
    pub target_part: String,
    pub conversion: String,
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L70
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawStat {
    /// if toggle is Some, the RawStat is option effect
    pub toggle: Option<String>,
    #[serde(default)]
    pub behavior: Behavior,
    pub bonuses: Vec<StatBonus>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Behavior {
    Merge,
    Modify,
    // only majid.json use overwrite
    Overwrite,
}
impl Default for Behavior {
    fn default() -> Self {
        Behavior::Merge
    }
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L79
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum StatBonus {
    Prop {
        abil: i32,
        name: String,
        value: f32,
    },
    Stat {
        #[serde(deserialize_with = "deserialize_stat_name")]
        name: StatName,
        value: i32,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StatName {
    DamMult(String),
    DefMult(String),
    HealMult(String),

    JH,
    SPD,
    DamRaw,
    SpPct1Final,
    SpPct2Final,
    SpPct3Final,

    // DamPct
    EDamPct,
    TDamPct,
    WDamPct,
    FDamPct,
    ADamPct,

    // DamAddMax
    EDamAddMax,
    TDamAddMax,
    WDamAddMax,
    FDamAddMax,
    ADamAddMax,

    // DamAddMin
    EDamAddMin,
    TDamAddMin,
    WDamAddMin,
    FDamAddMin,
    ADamAddMin,
}
fn deserialize_stat_name<'de, D>(deserializer: D) -> Result<StatName, D::Error>
where
    D: Deserializer<'de>,
{
    let type_str: String = Deserialize::deserialize(deserializer)?;

    if type_str.contains('.') {
        let parts: Vec<&str> = type_str.split('.').collect();
        let sub = parts[1..].join(".").to_string();
        return match parts[0] {
            "damMult" => Ok(StatName::DamMult(sub)),
            "defMult" => Ok(StatName::DefMult(sub)),
            "healMult" => Ok(StatName::HealMult(sub)),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown type: {}",
                type_str
            ))),
        };
    }
    match type_str.as_str() {
        "jh" => Ok(StatName::JH),
        "spd" => Ok(StatName::SPD),
        "damRaw" => Ok(StatName::DamRaw),
        "spPct1Final" => Ok(StatName::SpPct1Final),
        "spPct2Final" => Ok(StatName::SpPct2Final),
        "spPct3Final" => Ok(StatName::SpPct3Final),
        "eDamPct" => Ok(StatName::EDamPct),
        "tDamPct" => Ok(StatName::TDamPct),
        "wDamPct" => Ok(StatName::WDamPct),
        "fDamPct" => Ok(StatName::FDamPct),
        "aDamPct" => Ok(StatName::ADamPct),
        "eDamAddMax" => Ok(StatName::EDamAddMax),
        "tDamAddMax" => Ok(StatName::TDamAddMax),
        "wDamAddMax" => Ok(StatName::WDamAddMax),
        "fDamAddMax" => Ok(StatName::FDamAddMax),
        "aDamAddMax" => Ok(StatName::ADamAddMax),
        "eDamAddMin" => Ok(StatName::EDamAddMin),
        "tDamAddMin" => Ok(StatName::TDamAddMin),
        "wDamAddMin" => Ok(StatName::WDamAddMin),
        "fDamAddMin" => Ok(StatName::FDamAddMin),
        "aDamAddMin" => Ok(StatName::ADamAddMin),
        _ => Err(serde::de::Error::custom(format!(
            "Unknown type: {}",
            type_str
        ))),
    }
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/50ed4620bd0a4e3af7dd5646971c6dcd78e8b783/js/builder/atree.js#L85
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatScaling {
    // TODO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum NumberOrString {
    Number(f64),
    Text(String),
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    #[test]
    fn derivative_works() {
        let file = File::open("assets/atree.json")
            .expect("The file `atree.json` should exist in the folder assets.");
        let reader = BufReader::new(file);

        let _: AbilitiesMap = serde_json::from_reader(reader).unwrap();
    }
}
