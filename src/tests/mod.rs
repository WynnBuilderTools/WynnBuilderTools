use serde::Deserialize;

use crate::*;
use std::fs;

pub fn gen_test_apparels() -> Vec<ApparelsTestCase> {
    let file = fs::read("assets/apparels_test_case.json").unwrap();
    let items: Vec<ApparelsTestCase> = serde_json::from_slice(&file).unwrap();
    items
}

#[derive(Debug, Deserialize)]
pub struct ApparelsTestCase {
    #[serde(deserialize_with = "deserialize_apparels")]
    pub apparels: [Apparel; 8],
    #[serde(deserialize_with = "deserialize_weapon")]
    pub weapon: Weapon,
    pub skill_point: SkillPoints,
    pub skill_point_gap: Point,
    pub common_stat: CommonStat,
}
fn deserialize_apparels<'de, D>(deserializer: D) -> Result<[Apparel; 8], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let array: [Item; 8] = Deserialize::deserialize(deserializer)?;
    Ok(array.map(|v| Apparel::try_from(&v).unwrap()))
}
fn deserialize_weapon<'de, D>(deserializer: D) -> Result<Weapon, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let item: Item = Deserialize::deserialize(deserializer)?;
    Ok(Weapon::try_from(&item).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_test_apparels_works() {
        let x = gen_test_apparels();
        println!("{:?}", x);
    }
}
