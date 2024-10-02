use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    internalName: String,
    #[serde(rename = "type")]
    type_field: String,
    weaponType: Option<String>,
    attackSpeed: Option<String>,
    averageDps: Option<i32>,
    dropRestriction: String,
    requirements: Requirements,
    powderSlots: Option<i32>,
    lore: Option<String>,
    dropMeta: Option<DropMeta>,
    icon: Option<Icon>,
    identifications: Option<Identifications>,
    base: Base,
    rarity: String,
    armourType: String,
    armourMaterial: String,
    majorIds: Option<MajorIds>,
    restrictions: Option<String>,
    accessoryType: Option<String>,
    armourColor: Option<String>,
    identified: Option<bool>,
    allowCraftsman: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Requirements {
    agility: i32,
    quest: String,
    strength: i32,
    level: i32,
    dexterity: i32,
    classRequirement: String,
    defence: i32,
    intelligence: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DropMeta {
    name: Option<String>,
    #[serde(rename = "type")]
    type_field: Option<String>,
    coordinates: Option<[i32; 3]>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Icon {
    value: Option<Value>,
    format: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Value {
    id: Option<String>,
    customModelData: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Identifications {
    rawThunderDamage: Option<IdentificationStat>,
    elementalSpellDamage: Option<IdentificationStat>,
    rawEarthDamage: Option<IdentificationStat>,
    thunderDefence: Option<IdentificationStat>,
    rawNeutralSpellDamage: Option<IdentificationStat>,
    #[serde(rename = "raw2ndSpellCost")]
    rawSecondSpellCost: Option<IdentificationStat>,
    healthRegen: Option<IdentificationStat>,
    elementalDamage: Option<IdentificationStat>,
    waterSpellDamage: Option<IdentificationStat>,
    rawEarthMainAttackDamage: Option<IdentificationStat>,
    rawAirSpellDamage: Option<IdentificationStat>,
    #[serde(rename = "raw1stSpellCost")]
    rawFirstSpellCost: Option<IdentificationStat>,
    #[serde(rename = "2ndSpellCost")]
    secondSpellCost: Option<IdentificationStat>,
    jumpHeight: Option<IdentificationStat>,
    #[serde(rename = "3rdSpellCost")]
    thirdSpellCost: Option<IdentificationStat>,
    rawElementalDamage: Option<IdentificationStat>,
    lifeSteal: Option<IdentificationStat>,
    fireSpellDamage: Option<IdentificationStat>,
    earthDefence: Option<IdentificationStat>,
    rawDexterity: Option<IdentificationStat>,
    #[serde(rename = "4thSpellCost")]
    fourthSpellCost: Option<IdentificationStat>,
    rawStrength: Option<IdentificationStat>,
    fireDefence: Option<IdentificationStat>,
    raw3rdSpellCost: Option<IdentificationStat>,
    earthDamage: Option<IdentificationStat>,
    rawEarthSpellDamage: Option<IdentificationStat>,
    elementalMainAttackDamage: Option<IdentificationStat>,
    healingEfficiency: Option<IdentificationStat>,
    rawThunderMainAttackDamage: Option<IdentificationStat>,
    thunderSpellDamage: Option<IdentificationStat>,
    rawWaterDamage: Option<IdentificationStat>,
    rawMainAttackDamage: Option<IdentificationStat>,
    #[serde(rename = "1stSpellCost")]
    firstSpellCost: Option<IdentificationStat>,
    slowEnemy: Option<IdentificationStat>,
    sprint: Option<IdentificationStat>,
    fireMainAttackDamage: Option<IdentificationStat>,
    stealing: Option<IdentificationStat>,
    damage: Option<IdentificationStat>,
    airSpellDamage: Option<IdentificationStat>,
    rawElementalMainAttackDamage: Option<IdentificationStat>,
    manaRegen: Option<IdentificationStat>,
    rawAirDamage: Option<IdentificationStat>,
    airMainAttackDamage: Option<IdentificationStat>,
    neutralMainAttackDamage: Option<IdentificationStat>,
    rawThunderSpellDamage: Option<IdentificationStat>,
    rawFireMainAttackDamage: Option<IdentificationStat>,
    walkSpeed: Option<IdentificationStat>,
    rawDamage: Option<IdentificationStat>,
    airDefence: Option<IdentificationStat>,
    rawFireSpellDamage: Option<IdentificationStat>,
    rawAirMainAttackDamage: Option<IdentificationStat>,
    neutralDamage: Option<IdentificationStat>,
    rawNeutralMainAttackDamage: Option<IdentificationStat>,
    xpBonus: Option<IdentificationStat>,
    earthMainAttackDamage: Option<IdentificationStat>,
    waterDamage: Option<IdentificationStat>,
    raw4thSpellCost: Option<IdentificationStat>,
    spellDamage: Option<IdentificationStat>,
    manaSteal: Option<IdentificationStat>,
    lootBonus: Option<IdentificationStat>,
    waterDefence: Option<IdentificationStat>,
    rawHealth: Option<IdentificationStat>,
    airDamage: Option<IdentificationStat>,
    reflection: Option<IdentificationStat>,
    poison: Option<IdentificationStat>,
    weakenEnemy: Option<IdentificationStat>,
    rawNeutralDamage: Option<IdentificationStat>,
    rawElementalSpellDamage: Option<IdentificationStat>,
    rawSpellDamage: Option<IdentificationStat>,
    knockback: Option<IdentificationStat>,
    mainAttackDamage: Option<IdentificationStat>,
    rawFireDamage: Option<IdentificationStat>,
    thunderDamage: Option<IdentificationStat>,
    healthRegenRaw: Option<IdentificationStat>,
    earthSpellDamage: Option<IdentificationStat>,
    elementalDefence: Option<IdentificationStat>,
    rawAgility: Option<IdentificationStat>,
    rawIntelligence: Option<IdentificationStat>,
    thunderMainAttackDamage: Option<IdentificationStat>,
    exploding: Option<IdentificationStat>,
    sprintRegen: Option<IdentificationStat>,
    thorns: Option<IdentificationStat>,
    rawWaterSpellDamage: Option<IdentificationStat>,
    rawAttackSpeed: Option<IdentificationStat>,
    rawDefence: Option<IdentificationStat>,
    fireDamage: Option<IdentificationStat>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IdentificationStat {
    min: Option<i32>,
    raw: Option<i32>,
    max: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Base {
    baseAirDefence: i32,
    baseEarthDamage: i32,
    baseFireDefence: i32,
    baseWaterDamage: i32,
    baseDamage: i32,
    baseEarthDefence: i32,
    baseFireDamage: i32,
    baseAirDamage: i32,
    baseThunderDefence: i32,
    baseHealth: i32,
    baseWaterDefence: i32,
    baseThunderDamage: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MajorIds {
    major_id_name: Option<String>,
    major_id_description: Option<String>,
}