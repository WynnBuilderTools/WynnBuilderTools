use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Spell {
    pub name: String,
    pub id: i32,
    pub cost: i32,
    pub parts: Vec<DamagePart>,
}

impl Spell {
    pub fn new(name: String, id: i32, cost: i32, parts: Vec<DamagePart>) -> Self {
        Self {
            name,
            id,
            cost,
            parts,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DamagePart {
    pub name: String,
    pub dam_convert: DamagesConvert,
}
impl DamagePart {
    pub fn new(name: String, dam_convert: DamagesConvert) -> Self {
        Self { name, dam_convert }
    }
}
