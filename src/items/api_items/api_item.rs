use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiItems {
    pub items: HashMap<String, ApiItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiItem {
    #[serde(rename = "accessoryType")]
    pub accessory_type: Option<String>,
    #[serde(rename = "allowCraftsman")]
    pub allow_craftsman: Option<bool>,
    #[serde(rename = "armourColor")]
    pub armour_color: Option<String>,
    #[serde(rename = "armourMaterial")]
    pub armour_material: String,
    #[serde(rename = "armourType")]
    pub armour_type: String,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: Option<String>,
    #[serde(rename = "averageDps")]
    pub average_dps: Option<i32>,
    pub base: Base,
    #[serde(rename = "dropMeta")]
    pub drop_meta: Option<DropMeta>,
    #[serde(rename = "dropRestriction")]
    pub drop_restriction: String,
    pub icon: Icon,
    pub identifications: Option<Identifications>,
    pub identified: Option<bool>,
    #[serde(rename = "internalName")]
    pub internal_name: Option<String>,
    pub lore: Option<String>,
    #[serde(rename = "majorIds")]
    pub major_ids: Option<MajorId>,
    pub powder_slots: Option<i32>,
    pub rarity: String,
    pub requirements: Requirements,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "weaponType")]
    pub weapon_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Requirements {
    pub agility: Option<i32>,
    #[serde(rename = "classRequirement")]
    pub class_requirement: Option<String>,
    pub defence: Option<i32>,
    pub dexterity: Option<i32>,
    pub intelligence: Option<i32>,
    pub level: i32,
    pub quest: Option<String>,
    pub strength: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifications {
    #[serde(rename = "airDamage")]
    pub air_damage: Option<IdentificationStat>,
    #[serde(rename = "airDefence")]
    pub air_defence: Option<IdentificationStat>,
    #[serde(rename = "airMainAttackDamage")]
    pub air_main_attack_damage: Option<IdentificationStat>,
    #[serde(rename = "airSpellDamage")]
    pub air_spell_damage: Option<IdentificationStat>,
    pub damage: Option<IdentificationStat>,
    #[serde(rename = "earthDamage")]
    pub earth_damage: Option<IdentificationStat>,
    #[serde(rename = "earthDefence")]
    pub earth_defence: Option<IdentificationStat>,
    #[serde(rename = "elementalDamage")]
    pub elemental_damage: Option<IdentificationStat>,
    #[serde(rename = "elementalMainAttackDamage")]
    pub elemental_main_attack_damage: Option<IdentificationStat>,
    #[serde(rename = "elementalSpellDamage")]
    pub elemental_spell_damage: Option<IdentificationStat>,
    #[serde(rename = "xpBonus")]
    pub exp_bonus: Option<IdentificationStat>,
    #[serde(rename = "fireDamage")]
    pub fire_damage: Option<IdentificationStat>,
    #[serde(rename = "fireDefence")]
    pub fire_defence: Option<IdentificationStat>,
    #[serde(rename = "fireMainAttackDamage")]
    pub fire_main_attack_damage: Option<IdentificationStat>,
    #[serde(rename = "fireSpellDamage")]
    pub fire_spell_damage: Option<IdentificationStat>,
    #[serde(rename = "1stSpellCost")]
    pub first_spell_cost: Option<IdentificationStat>,
    #[serde(rename = "4thSpellCost")]
    pub fourth_spell_cost: Option<IdentificationStat>,
    #[serde(rename = "healingEfficiency")]
    pub healing_efficiency: Option<IdentificationStat>,
    #[serde(rename = "healthRegen")]
    pub health_regen_pct: Option<IdentificationStat>,
    #[serde(rename = "jumpHeight")]
    pub jump_height: Option<IdentificationStat>,
    #[serde(rename = "lifeSteal")]
    pub life_steal: Option<IdentificationStat>,
    #[serde(rename = "mainAttackDamage")]
    pub main_attack_damage: Option<IdentificationStat>,
    #[serde(rename = "manaRegen")]
    pub mana_regen: Option<IdentificationStat>,
    #[serde(rename = "manaSteal")]
    pub mana_steal: Option<IdentificationStat>,
    #[serde(rename = "neutralDamage")]
    pub neutral_damage: Option<IdentificationStat>,
    #[serde(rename = "neutralMainAttackDamage")]
    pub neutral_main_attack_damage: Option<IdentificationStat>,
    pub poison: Option<IdentificationStat>,
    #[serde(rename = "rawAgility")]
    pub raw_agility: Option<i32>,
    #[serde(rename = "rawAirDamage")]
    pub raw_air_damage: Option<IdentificationStat>,
    #[serde(rename = "rawAirMainAttackDamage")]
    pub raw_air_main_attack_damage: Option<IdentificationStat>,

    #[serde(rename = "rawAirSpellDamage")]
    pub raw_air_spell_damage: Option<IdentificationStat>,
    #[serde(rename = "rawDamage")]
    pub raw_damage: Option<IdentificationStat>,
    #[serde(rename = "rawDefence")]
    pub raw_defence: Option<i32>,
    #[serde(rename = "rawDexterity")]
    pub raw_dexterity: Option<i32>,
    #[serde(rename = "rawEarthDamage")]
    pub raw_earth_damage: Option<IdentificationStat>,
    #[serde(rename = "rawEarthMainAttackDamage")]
    pub raw_earth_main_attack_damage: Option<IdentificationStat>,
    #[serde(rename = "rawEarthSpellDamage")]
    pub raw_earth_spell_damage: Option<IdentificationStat>,
    #[serde(rename = "rawElementalDamage")]
    pub raw_elemental_damage: Option<IdentificationStat>,
    #[serde(rename = "rawElementalMainAttackDamage")]
    pub raw_elemental_main_attack_damage: Option<IdentificationStat>,
    #[serde(rename = "rawFireMainAttackDamage")]
    pub raw_fire_main_attack_damage: Option<IdentificationStat>,
    #[serde(rename = "rawFireSpellDamage")]
    pub raw_fire_spell_damage: Option<IdentificationStat>,
    #[serde(rename = "raw1stSpellCost")]
    pub raw_first_spell_cost: Option<IdentificationStat>,
    #[serde(rename = "rawHealth")]
    pub raw_health: Option<IdentificationStat>,
    #[serde(rename = "healthRegenRaw")]
    pub raw_health_regen: Option<IdentificationStat>,
    #[serde(rename = "rawIntelligence")]
    pub raw_intelligence: Option<i32>,
    #[serde(rename = "rawMainAttackDamage")]
    pub raw_main_attack_damage: Option<IdentificationStat>,
    #[serde(rename = "rawNeutralSpellDamage")]
    pub raw_neutral_spell_damage: Option<IdentificationStat>,
    #[serde(rename = "raw2ndSpellCost")]
    pub raw_second_spell_cost: Option<IdentificationStat>,
    #[serde(rename = "rawSpellDamage")]
    pub raw_spell_damage: Option<IdentificationStat>,
    #[serde(rename = "rawStrength")]
    pub raw_strength: Option<i32>,
    #[serde(rename = "raw3rdSpellCost")]
    pub raw_third_spell_cost: Option<IdentificationStat>,
    #[serde(rename = "rawThunderDamage")]
    pub raw_thunder_damage: Option<IdentificationStat>,
    #[serde(rename = "rawThunderMainAttackDamage")]
    pub raw_thunder_main_attack_damage: Option<IdentificationStat>,
    #[serde(rename = "rawThunderSpellDamage")]
    pub raw_thunder_spell_damage: Option<IdentificationStat>,
    #[serde(rename = "rawWaterDamage")]
    pub raw_water_damage: Option<IdentificationStat>,
    #[serde(rename = "2ndSpellCost")]
    pub second_spell_cost: Option<IdentificationStat>,
    #[serde(rename = "slowEnemy")]
    pub slow_enemy: Option<IdentificationStat>,
    #[serde(rename = "spellDamage")]
    pub spell_damage: Option<IdentificationStat>,
    pub sprint: Option<IdentificationStat>,
    pub stealing: Option<IdentificationStat>,
    #[serde(rename = "3rdSpellCost")]
    pub third_spell_cost: Option<IdentificationStat>,
    #[serde(rename = "thunderDamage")]
    pub thunder_damage: Option<IdentificationStat>,
    #[serde(rename = "thunderDefence")]
    pub thunder_defence: Option<IdentificationStat>,
    #[serde(rename = "thunderSpellDamage")]
    pub thunder_spell_damage: Option<IdentificationStat>,
    #[serde(rename = "walkSpeed")]
    pub walk_speed: Option<IdentificationStat>,
    #[serde(rename = "waterDamage")]
    pub water_damage: Option<IdentificationStat>,
    #[serde(rename = "waterDefence")]
    pub water_defence: Option<IdentificationStat>,
    #[serde(rename = "waterSpellDamage")]
    pub water_spell_damage: Option<IdentificationStat>,
}

impl Default for Identifications {
    fn default() -> Self {
        Self {
            poison: Default::default(),
            raw_thunder_damage: Default::default(),
            raw_health: Default::default(),
            elemental_spell_damage: Default::default(),
            raw_earth_damage: Default::default(),
            thunder_defence: Default::default(),
            raw_neutral_spell_damage: Default::default(),
            raw_second_spell_cost: Default::default(),
            health_regen_pct: Default::default(),
            elemental_damage: Default::default(),
            water_spell_damage: Default::default(),
            raw_earth_main_attack_damage: Default::default(),
            raw_air_spell_damage: Default::default(),
            raw_first_spell_cost: Default::default(),
            second_spell_cost: Default::default(),
            jump_height: Default::default(),
            third_spell_cost: Default::default(),
            raw_elemental_damage: Default::default(),
            life_steal: Default::default(),
            fire_spell_damage: Default::default(),
            earth_defence: Default::default(),
            raw_dexterity: Default::default(),
            fourth_spell_cost: Default::default(),
            raw_strength: Default::default(),
            fire_defence: Default::default(),
            raw_third_spell_cost: Default::default(),
            earth_damage: Default::default(),
            raw_earth_spell_damage: Default::default(),
            elemental_main_attack_damage: Default::default(),
            healing_efficiency: Default::default(),
            raw_thunder_main_attack_damage: Default::default(),
            thunder_spell_damage: Default::default(),
            raw_water_damage: Default::default(),
            raw_main_attack_damage: Default::default(),
            first_spell_cost: Default::default(),
            slow_enemy: Default::default(),
            sprint: Default::default(),
            fire_main_attack_damage: Default::default(),
            stealing: Default::default(),
            damage: Default::default(),
            air_spell_damage: Default::default(),
            raw_elemental_main_attack_damage: Default::default(),
            mana_regen: Default::default(),
            raw_air_damage: Default::default(),
            air_main_attack_damage: Default::default(),
            neutral_main_attack_damage: Default::default(),
            raw_thunder_spell_damage: Default::default(),
            raw_fire_main_attack_damage: Default::default(),
            walk_speed: Default::default(),
            raw_damage: Default::default(),
            air_defence: Default::default(),
            raw_fire_spell_damage: Default::default(),
            raw_air_main_attack_damage: Default::default(),
            neutral_damage: Default::default(),
            exp_bonus: Default::default(),
            raw_spell_damage: Default::default(),
            spell_damage: Default::default(),
            raw_health_regen: Default::default(),
            mana_steal: Default::default(),
            water_defence: Default::default(),
            raw_intelligence: Default::default(),
            raw_agility: Default::default(),
            raw_defence: Default::default(),
            thunder_damage: Default::default(),
            water_damage: Default::default(),
            fire_damage: Default::default(),
            air_damage: Default::default(),
            main_attack_damage: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentificationStat {
    pub min: i32,
    pub raw: i32,
    pub max: i32,
}

impl IdentificationStat {
    pub fn min(&self) -> i32 {
        self.min
    }

    pub fn raw(&self) -> i32 {
        self.raw
    }

    pub fn max(&self) -> i32 {
        self.max
    }

    pub fn new(min: i32, raw: i32, max: i32) -> Self {
        IdentificationStat { min, raw, max }
    }

    pub fn zero() -> Self {
        IdentificationStat::new(0, 0, 0)
    }
}

impl Default for IdentificationStat {
    fn default() -> Self {
        IdentificationStat::zero()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Base {
    #[serde(rename = "baseAirDamage")]
    pub base_air_damage: Option<IdentificationStat>,
    #[serde(rename = "baseAirDefence")]
    pub base_air_defence: Option<i32>,
    #[serde(rename = "baseDamage")]
    pub base_damage: Option<IdentificationStat>,
    #[serde(rename = "baseEarthDamage")]
    pub base_earth_damage: Option<IdentificationStat>,
    #[serde(rename = "baseEarthDefence")]
    pub base_earth_defence: Option<i32>,
    #[serde(rename = "baseFireDamage")]
    pub base_fire_damage: Option<IdentificationStat>,
    #[serde(rename = "baseFireDefence")]
    pub base_fire_defence: Option<i32>,
    #[serde(rename = "baseHealth")]
    pub base_health: Option<i32>,
    #[serde(rename = "baseThunderDamage")]
    pub base_thunder_damage: Option<IdentificationStat>,
    #[serde(rename = "baseThunderDefence")]
    pub base_thunder_defence: Option<i32>,
    #[serde(rename = "baseWaterDamage")]
    pub base_water_damage: Option<IdentificationStat>,
    #[serde(rename = "baseWaterDefence")]
    pub base_water_defence: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MajorId {
    #[serde(flatten)]
    pub data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DropMeta {
    pub coordinates: Option<[i32; 3]>,
    pub event: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    pub format: Option<String>,
    pub value: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    #[serde(rename = "customModelData")]
    pub custom_model_data: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
}

impl Copy for IdentificationStat {}

impl Clone for IdentificationStat {
    fn clone(&self) -> Self {
        *self
    }
}