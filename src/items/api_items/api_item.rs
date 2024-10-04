extern crate schemafy;
use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_with::formats::PreferOne;
use serde_with::serde_as;
use serde_with::OneOrMany;

pub type ApiItems = HashMap<String, ApiItem>;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum AccessoryType {
    #[serde(rename = "bracelet")]
    Bracelet,
    #[serde(rename = "necklace")]
    Necklace,
    #[serde(rename = "ring")]
    Ring,
}

impl Display for AccessoryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccessoryType::Bracelet => write!(f, "bracelet"),
            AccessoryType::Necklace => write!(f, "necklace"),
            AccessoryType::Ring => write!(f, "ring"),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ArmourMaterial {
    #[serde(rename = "chain")]
    Chain,
    #[serde(rename = "diamond")]
    Diamond,
    #[serde(rename = "golden")]
    Golden,
    #[serde(rename = "iron")]
    Iron,
    #[serde(rename = "leather")]
    Leather,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ArmourType {
    #[serde(rename = "boots")]
    Boots,
    #[serde(rename = "chestplate")]
    Chestplate,
    #[serde(rename = "helmet")]
    Helmet,
    #[serde(rename = "leggings")]
    Leggings,
}

impl Display for ArmourType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArmourType::Boots => write!(f, "boots"),
            ArmourType::Chestplate => write!(f, "chestplate"),
            ArmourType::Helmet => write!(f, "helmet"),
            ArmourType::Leggings => write!(f, "leggings"),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum AttackSpeed {
    #[serde(rename = "fast")]
    Fast,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "super_fast")]
    SuperFast,
    #[serde(rename = "super_slow")]
    SuperSlow,
    #[serde(rename = "very_fast")]
    VeryFast,
    #[serde(rename = "very_slow")]
    VerySlow,
}

impl Display for AttackSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttackSpeed::Fast => write!(f, "Fast"),
            AttackSpeed::Normal => write!(f, "Normal"),
            AttackSpeed::Slow => write!(f, "Slow"),
            AttackSpeed::SuperFast => write!(f, "Super Fast"),
            AttackSpeed::SuperSlow => write!(f, "Super Slow"),
            AttackSpeed::VeryFast => write!(f, "Very Fast"),
            AttackSpeed::VerySlow => write!(f, "Very Slow"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Base {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_air_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_air_defence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_earth_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_earth_defence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fire_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_fire_defence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_health: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_thunder_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_thunder_defence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_water_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_water_defence: Option<i32>,
}

#[derive(Clone, Copy, PartialEq, Debug, Deserialize, Serialize)]
pub struct Stat {
    pub max: i32,
    pub min: i32,
    pub raw: i32,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StatOrInt {
    Stat(Stat),
    Int(i32),
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ClassRequirement {
    #[serde(rename = "archer")]
    Archer,
    #[serde(rename = "assassin")]
    Assassin,
    #[serde(rename = "mage")]
    Mage,
    #[serde(rename = "shaman")]
    Shaman,
    #[serde(rename = "warrior")]
    Warrior,
}

#[serde_as]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DropMeta {
    pub coordinates: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<Event>,
    pub name: String,
    #[serde_as(as = "OneOrMany<_, PreferOne>")]
    #[serde(rename = "type")]
    pub type_field: Vec<DropMetaType>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum DropRestriction {
    #[serde(rename = "dungeon")]
    Dungeon,
    #[serde(rename = "lootchest")]
    Lootchest,
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "normal")]
    Normal,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Event {
    #[serde(rename = "bonfire")]
    Bonfire,
    #[serde(rename = "heroes")]
    Heroes,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Format {
    #[serde(rename = "attribute")]
    Attribute,
    #[serde(rename = "legacy")]
    Legacy,
    #[serde(rename = "skin")]
    Skin,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename = "ID")]
pub enum Id {
    #[serde(rename = "minecraft:iron_horse_armor")]
    MinecraftIronHorseArmor,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Icon {
    pub format: Format,
    pub value: ValueUnion,
}

#[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Identifications {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_defence: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub air_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earth_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earth_defence: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earth_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earth_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elemental_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elemental_defence: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elemental_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elemental_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploding: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_defence: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "1stSpellCost")]
    pub first_spell_cost: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "4thSpellCost")]
    pub fourth_spell_cost: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub healing_efficiency: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_regen: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_regen_raw: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jump_height: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub knockback: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub life_steal: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loot_bonus: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana_regen: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mana_steal: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neutral_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poison: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "raw1stSpellCost")]
    pub raw_1st_spell_cost: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "raw2ndSpellCost")]
    pub raw_2nd_spell_cost: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "raw3rdSpellCost")]
    pub raw_3rd_spell_cost: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "raw4thSpellCost")]
    pub raw_4th_spell_cost: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_agility: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_air_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_air_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_air_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_attack_speed: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_defence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_dexterity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_earth_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_earth_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_earth_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_elemental_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_elemental_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_elemental_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_fire_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_fire_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_fire_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_health: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_intelligence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_neutral_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_neutral_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_neutral_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_strength: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_thunder_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_thunder_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_thunder_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_water_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_water_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflection: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "2ndSpellCost")]
    pub second_spell_cost: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_enemy: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprint: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sprint_regen: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stealing: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "3rdSpellCost")]
    pub third_spell_cost: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thorns: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thunder_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thunder_defence: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thunder_main_attack_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thunder_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub walk_speed: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_defence: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub water_spell_damage: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weaken_enemy: Option<StatOrInt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xp_bonus: Option<StatOrInt>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum DropMetaType {
    #[serde(rename = "altar")]
    Altar,
    #[serde(rename = "challenge")]
    Challenge,
    #[serde(rename = "dungeon")]
    Dungeon,
    #[serde(rename = "dungeonMerchant")]
    DungeonMerchant,
    #[serde(rename = "lootrun")]
    Lootrun,
    #[serde(rename = "merchant")]
    Merchant,
    #[serde(rename = "miniboss")]
    Miniboss,
    #[serde(rename = "quest")]
    Quest,
    #[serde(rename = "raid")]
    Raid,
    #[serde(rename = "event")]
    Event,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Rarity {
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "fabled")]
    Fabled,
    #[serde(rename = "legendary")]
    Legendary,
    #[serde(rename = "mythic")]
    Mythic,
    #[serde(rename = "rare")]
    Rare,
    #[serde(rename = "set")]
    Set,
    #[serde(rename = "unique")]
    Unique,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Requirements {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agility: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "classRequirement")]
    pub class_requirement: Option<ClassRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defence: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dexterity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intelligence: Option<i32>,
    pub level: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<i32>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum Restrictions {
    #[serde(rename = "quest item")]
    QuestItem,
    #[serde(rename = "untradable")]
    Untradable,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum EventType {
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "merchant")]
    Merchant,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ValueClass {
    #[serde(rename = "customModelData")]
    pub custom_model_data: String,
    pub id: Id,
    pub name: String,
}
pub type ValueUnion = serde_json::Value;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum WeaponType {
    #[serde(rename = "bow")]
    Bow,
    #[serde(rename = "dagger")]
    Dagger,
    #[serde(rename = "relik")]
    Relik,
    #[serde(rename = "spear")]
    Spear,
    #[serde(rename = "wand")]
    Wand,
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponType::Bow => write!(f, "bow"),
            WeaponType::Dagger => write!(f, "dagger"),
            WeaponType::Relik => write!(f, "relik"),
            WeaponType::Spear => write!(f, "spear"),
            WeaponType::Wand => write!(f, "wand"),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub enum ItemType {
    #[serde(rename = "accessory")]
    Accessory,
    #[serde(rename = "armour")]
    Armour,
    #[serde(rename = "weapon")]
    Weapon,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessory_type: Option<AccessoryType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_craftsman: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armour_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armour_material: Option<ArmourMaterial>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub armour_type: Option<ArmourType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack_speed: Option<AttackSpeed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_dps: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<Base>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_meta: Option<DropMeta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_restriction: Option<DropRestriction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifications: Option<Identifications>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identified: Option<bool>,
    pub internal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lore: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_ids: Option<::std::collections::BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub powder_slots: Option<i32>,
    pub rarity: Rarity,
    pub requirements: Requirements,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Restrictions>,
    #[serde(rename = "type")]
    pub type_field: ItemType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weapon_type: Option<WeaponType>,
}

impl Stat {
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
        Stat { min, raw, max }
    }

    pub fn zero() -> Self {
        Stat::new(0, 0, 0)
    }
}

impl Default for Stat {
    fn default() -> Self {
        Stat::zero()
    }
}

impl Display for Rarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rarity::Common => write!(f, "Common"),
            Rarity::Unique => write!(f, "Unique"),
            Rarity::Rare => write!(f, "Rare"),
            Rarity::Legendary => write!(f, "Legendary"),
            Rarity::Mythic => write!(f, "Mythic"),
            Rarity::Fabled => write!(f, "Fabled"),
            Rarity::Set => write!(f, "Set"),
        }
    }
}

impl Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Accessory => write!(f, "Accessory"),
            ItemType::Armour => write!(f, "Armour"),
            ItemType::Weapon => write!(f, "Weapon"),
        }
    }
}
