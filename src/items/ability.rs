use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Abilities {
    pub mage: Vec<ATreeNodeData>,
    pub warrior: Vec<ATreeNodeData>,
    pub archer: Vec<ATreeNodeData>,
    pub assassin: Vec<ATreeNodeData>,
    pub shaman: Vec<ATreeNodeData>,
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/7afed640451f2a085640b6c5e7259fcc827119ef/js/builder/atree.js#L15
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ATreeNodeData {
    pub display_name: String,
    pub id: i32,
    pub base_abil: Option<i32>,
    pub cost: i32,
    pub effects: Vec<Effect>,
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/7afed640451f2a085640b6c5e7259fcc827119ef/js/builder/atree.js#L36
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

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/7afed640451f2a085640b6c5e7259fcc827119ef/js/damage_calc.js#L233
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplaceSpell {
    name: String,
    cost: Option<i32>,
    base_spell: i32,
    spell_type: Option<String>,
    scaling: Option<String>,
    use_atkspd: Option<bool>,
    display: Option<String>,
    parts: Vec<Part>,
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/7afed640451f2a085640b6c5e7259fcc827119ef/js/damage_calc.js#L252
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum Part {
    Damage {
        name: String,
        multipliers: [f64; 6],
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

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/7afed640451f2a085640b6c5e7259fcc827119ef/js/builder/atree.js#L43
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddSpellProp {
    base_spell: i32,
    target_part: Option<String>,
    behavior: Option<Behavior>,
    cost: Option<i32>,
    multipliers: Option<Vec<f32>>,
    power: Option<f32>,
    hits: Option<HashMap<String, NumberOrString>>,
    display: Option<String>,
    hide: Option<bool>,
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/7afed640451f2a085640b6c5e7259fcc827119ef/js/builder/atree.js#L63
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConvertSpellConv {
    pub base_spell: i32,
    pub target_part: String,
    pub conversion: String,
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/7afed640451f2a085640b6c5e7259fcc827119ef/js/builder/atree.js#L70
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
}
impl Default for Behavior {
    fn default() -> Self {
        Behavior::Merge
    }
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/7afed640451f2a085640b6c5e7259fcc827119ef/js/builder/atree.js#L79
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

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/7afed640451f2a085640b6c5e7259fcc827119ef/js/builder/atree.js#L85
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatScaling {
    // TODO
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum NumberOrString {
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

        let _: Abilities = serde_json::from_reader(reader).unwrap();
    }
}
