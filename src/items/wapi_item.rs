use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

use super::*;

pub type WApiItems = HashMap<String, WApiItem>;

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum AccessoryType {
    Bracelet,
    Necklace,
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
#[serde(rename_all = "camelCase")]
pub enum ArmourMaterial {
    Chain,
    Diamond,
    Golden,
    Gold,
    Iron,
    Leather,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ArmourType {
    Boots,
    Chestplate,
    Helmet,
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
#[serde(rename_all = "snake_case")]
pub enum AttackSpeed {
    Fast,
    Normal,
    Slow,
    SuperFast,
    SuperSlow,
    VeryFast,
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
    pub base_air_damage: Option<StatOrInt>,
    pub base_air_defence: Option<i32>,
    pub base_damage: Option<StatOrInt>,
    pub base_earth_damage: Option<StatOrInt>,
    pub base_earth_defence: Option<i32>,
    pub base_fire_damage: Option<StatOrInt>,
    pub base_fire_defence: Option<i32>,
    pub base_health: Option<i32>,
    pub base_thunder_damage: Option<StatOrInt>,
    pub base_thunder_defence: Option<i32>,
    pub base_water_damage: Option<StatOrInt>,
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
impl StatOrInt {
    pub fn max(&self) -> i32 {
        match self {
            StatOrInt::Stat(stat) => stat.max,
            StatOrInt::Int(value) => *value,
        }
    }
    pub fn min(&self) -> i32 {
        match self {
            StatOrInt::Stat(stat) => stat.min,
            StatOrInt::Int(value) => *value,
        }
    }
    pub fn must_int(&self) -> Result<i32, String> {
        match self {
            StatOrInt::Stat(_) => Err("StatOrInt must be int".to_string()),
            StatOrInt::Int(value) => Ok(*value),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ClassRequirement {
    Archer,
    Assassin,
    Mage,
    Shaman,
    Warrior,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct DropMeta {
    pub coordinates: Vec<i32>,
    pub event: Option<Event>,
    pub name: String,
    pub r#type: DropMetaTypeOrVec,
}
#[derive(Serialize, Clone, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum DropMetaTypeOrVec {
    Single(DropMetaType),
    Multiple(Vec<DropMetaType>),
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DropRestriction {
    Dungeon,
    Lootchest,
    Never,
    Normal,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Event {
    Bonfire,
    Heroes,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Format {
    Attribute,
    Legacy,
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
    pub air_damage: Option<StatOrInt>,
    pub air_defence: Option<StatOrInt>,
    pub air_main_attack_damage: Option<StatOrInt>,
    pub air_spell_damage: Option<StatOrInt>,
    pub damage: Option<StatOrInt>,
    pub earth_damage: Option<StatOrInt>,
    pub earth_defence: Option<StatOrInt>,
    pub earth_main_attack_damage: Option<StatOrInt>,
    pub earth_spell_damage: Option<StatOrInt>,
    pub elemental_damage: Option<StatOrInt>,
    pub elemental_defence: Option<StatOrInt>,
    pub elemental_main_attack_damage: Option<StatOrInt>,
    pub elemental_spell_damage: Option<StatOrInt>,
    pub exploding: Option<StatOrInt>,
    pub fire_damage: Option<StatOrInt>,
    pub fire_defence: Option<StatOrInt>,
    pub fire_main_attack_damage: Option<StatOrInt>,
    pub fire_spell_damage: Option<StatOrInt>,
    #[serde(rename = "1stSpellCost")]
    pub first_spell_cost: Option<StatOrInt>,
    #[serde(rename = "4thSpellCost")]
    pub fourth_spell_cost: Option<StatOrInt>,
    pub healing_efficiency: Option<StatOrInt>,
    pub health_regen: Option<StatOrInt>,
    pub health_regen_raw: Option<StatOrInt>,
    pub jump_height: Option<StatOrInt>,
    pub knockback: Option<StatOrInt>,
    pub life_steal: Option<StatOrInt>,
    pub loot_bonus: Option<StatOrInt>,
    pub main_attack_damage: Option<StatOrInt>,
    pub mana_regen: Option<StatOrInt>,
    pub mana_steal: Option<StatOrInt>,
    pub neutral_damage: Option<StatOrInt>,
    pub neutral_main_attack_damage: Option<StatOrInt>,
    pub poison: Option<StatOrInt>,
    #[serde(rename = "raw1stSpellCost")]
    pub raw_1st_spell_cost: Option<StatOrInt>,
    #[serde(rename = "raw2ndSpellCost")]
    pub raw_2nd_spell_cost: Option<StatOrInt>,
    #[serde(rename = "raw3rdSpellCost")]
    pub raw_3rd_spell_cost: Option<StatOrInt>,
    #[serde(rename = "raw4thSpellCost")]
    pub raw_4th_spell_cost: Option<StatOrInt>,
    pub raw_agility: Option<StatOrInt>,
    pub raw_air_damage: Option<StatOrInt>,
    pub raw_air_main_attack_damage: Option<StatOrInt>,
    pub raw_air_spell_damage: Option<StatOrInt>,
    pub raw_attack_speed: Option<StatOrInt>,
    pub raw_damage: Option<StatOrInt>,
    pub raw_defence: Option<StatOrInt>,
    pub raw_dexterity: Option<StatOrInt>,
    pub raw_earth_damage: Option<StatOrInt>,
    pub raw_earth_main_attack_damage: Option<StatOrInt>,
    pub raw_earth_spell_damage: Option<StatOrInt>,
    pub raw_elemental_damage: Option<StatOrInt>,
    pub raw_elemental_main_attack_damage: Option<StatOrInt>,
    pub raw_elemental_spell_damage: Option<StatOrInt>,
    pub raw_fire_damage: Option<StatOrInt>,
    pub raw_fire_main_attack_damage: Option<StatOrInt>,
    pub raw_fire_spell_damage: Option<StatOrInt>,
    pub raw_health: Option<StatOrInt>,
    pub raw_intelligence: Option<StatOrInt>,
    pub raw_main_attack_damage: Option<StatOrInt>,
    pub raw_neutral_damage: Option<StatOrInt>,
    pub raw_neutral_main_attack_damage: Option<StatOrInt>,
    pub raw_neutral_spell_damage: Option<StatOrInt>,
    pub raw_spell_damage: Option<StatOrInt>,
    pub raw_strength: Option<StatOrInt>,
    pub raw_thunder_damage: Option<StatOrInt>,
    pub raw_thunder_main_attack_damage: Option<StatOrInt>,
    pub raw_thunder_spell_damage: Option<StatOrInt>,
    pub raw_water_damage: Option<StatOrInt>,
    pub raw_water_spell_damage: Option<StatOrInt>,
    pub reflection: Option<StatOrInt>,
    #[serde(rename = "2ndSpellCost")]
    pub second_spell_cost: Option<StatOrInt>,
    pub slow_enemy: Option<StatOrInt>,
    pub spell_damage: Option<StatOrInt>,
    pub sprint: Option<StatOrInt>,
    pub sprint_regen: Option<StatOrInt>,
    pub stealing: Option<StatOrInt>,
    #[serde(rename = "3rdSpellCost")]
    pub third_spell_cost: Option<StatOrInt>,
    pub thorns: Option<StatOrInt>,
    pub thunder_damage: Option<StatOrInt>,
    pub thunder_defence: Option<StatOrInt>,
    pub thunder_main_attack_damage: Option<StatOrInt>,
    pub thunder_spell_damage: Option<StatOrInt>,
    pub walk_speed: Option<StatOrInt>,
    pub water_damage: Option<StatOrInt>,
    pub water_defence: Option<StatOrInt>,
    pub water_spell_damage: Option<StatOrInt>,
    pub weaken_enemy: Option<StatOrInt>,
    pub xp_bonus: Option<StatOrInt>,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DropMetaType {
    Altar,
    Challenge,
    Dungeon,
    DungeonMerchant,
    Lootrun,
    Merchant,
    Miniboss,
    Guild,
    Quest,
    Raid,
    Event,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Rarity {
    Common,
    Fabled,
    Legendary,
    Mythic,
    Rare,
    Set,
    Unique,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Requirements {
    pub agility: Option<i32>,
    #[serde(rename = "classRequirement")]
    pub class_requirement: Option<ClassRequirement>,
    pub defence: Option<i32>,
    pub dexterity: Option<i32>,
    pub intelligence: Option<i32>,
    pub level: i32,
    pub quest: Option<String>,
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
#[serde(rename_all = "camelCase")]
pub enum EventType {
    Event,
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
#[serde(rename_all = "camelCase")]
pub enum WeaponType {
    Bow,
    Dagger,
    Relik,
    Spear,
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
#[serde(rename_all = "camelCase")]
pub enum ItemType {
    Accessory,
    Armour,
    Weapon,
    Material,
    Tool,
    Ingredient,
    Charm,
    Tome,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WApiItem {
    pub accessory_type: Option<AccessoryType>,
    pub allow_craftsman: Option<bool>,
    pub armour_color: Option<String>,
    pub armour_material: Option<ArmourMaterial>,
    pub armour_type: Option<ArmourType>,
    pub attack_speed: Option<AttackSpeed>,
    pub average_dps: Option<i32>,
    pub base: Option<Base>,
    pub drop_meta: Option<DropMeta>,
    pub drop_restriction: Option<DropRestriction>,
    pub icon: Option<Icon>,
    pub identifications: Option<Identifications>,
    pub identified: Option<bool>,
    pub internal_name: String,
    pub lore: Option<String>,
    pub major_ids: Option<::std::collections::BTreeMap<String, String>>,
    pub powder_slots: Option<i32>,
    pub rarity: Option<Rarity>,
    pub requirements: Requirements,
    pub restrictions: Option<Restrictions>,
    pub r#type: Option<ItemType>,
    pub weapon_type: Option<WeaponType>,
}
impl WApiItem {
    pub fn item_type(&self) -> Result<String, String> {
        let item_type = if let Some(armour_type) = &self.armour_type {
            armour_type.to_string()
        } else if let Some(accessory_type) = &self.accessory_type {
            accessory_type.to_string()
        } else if let Some(weapon_type) = &self.weapon_type {
            weapon_type.to_string()
        } else {
            return Err("Invalid value for wapi item type".to_string());
        };
        Ok(item_type)
    }
    pub fn hp_bonus_max(&self) -> i32 {
        self.identifications
            .as_ref()
            .and_then(|ids| ids.raw_health)
            .map(|raw_health| raw_health.max())
            .unwrap_or(0)
    }
    pub fn hp_bonus_min(&self) -> i32 {
        self.identifications
            .as_ref()
            .and_then(|ids| ids.raw_health)
            .map(|raw_health| raw_health.max())
            .unwrap_or(0)
    }
    pub fn add(&self) -> Result<Point, String> {
        fn extract_value<F>(identifications: &Identifications, extractor: F) -> Result<i16, String>
        where
            F: Fn(&Identifications) -> Option<StatOrInt>,
        {
            match extractor(identifications).map(|v| v.must_int()) {
                Some(v) => v
                    .map(|v| v as i16)
                    .or_else(|_| Err("item add point is range".to_string())),
                None => Ok(0),
            }
        }

        let Some(ids) = self.identifications.as_ref() else {
            return Ok(Point::default());
        };
        Ok(Point::new(
            extract_value(&ids, |v| v.raw_strength)?,
            extract_value(&ids, |v| v.raw_dexterity)?,
            extract_value(&ids, |v| v.raw_intelligence)?,
            extract_value(&ids, |v| v.raw_defence)?,
            extract_value(&ids, |v| v.raw_agility)?,
        ))
    }
    pub fn req(&self) -> Point {
        self.requirements.clone().into()
    }
    pub fn def(&self) -> Point {
        let Some(base) = self.base.as_ref() else {
            return Point::default();
        };
        Point::new(
            base.base_earth_defence.unwrap_or(0) as i16,
            base.base_thunder_defence.unwrap_or(0) as i16,
            base.base_water_defence.unwrap_or(0) as i16,
            base.base_fire_defence.unwrap_or(0) as i16,
            base.base_air_defence.unwrap_or(0) as i16,
        )
    }
    pub fn def_pct_max(&self) -> Point {
        let Some(ids) = self.identifications.as_ref() else {
            return Point::default();
        };
        Point::new(
            ids.earth_defence.map(|v| v.max()).unwrap_or(0) as i16,
            ids.thunder_defence.map(|v| v.max()).unwrap_or(0) as i16,
            ids.water_defence.map(|v| v.max()).unwrap_or(0) as i16,
            ids.fire_defence.map(|v| v.max()).unwrap_or(0) as i16,
            ids.air_defence.map(|v| v.max()).unwrap_or(0) as i16,
        )
    }
    pub fn def_pct_min(&self) -> Point {
        let Some(ids) = self.identifications.as_ref() else {
            return Point::default();
        };
        Point::new(
            ids.earth_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.thunder_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.water_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.fire_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.air_damage.map(|v| v.min()).unwrap_or(0) as i16,
        )
    }
    pub fn dam_pct_max(&self) -> Dam {
        let Some(ids) = self.identifications.as_ref() else {
            return Dam::default();
        };
        Dam::new(
            ids.neutral_damage.map(|v| v.max()).unwrap_or(0) as i16,
            ids.earth_damage.map(|v| v.max()).unwrap_or(0) as i16,
            ids.thunder_damage.map(|v| v.max()).unwrap_or(0) as i16,
            ids.water_damage.map(|v| v.max()).unwrap_or(0) as i16,
            ids.fire_damage.map(|v| v.max()).unwrap_or(0) as i16,
            ids.air_damage.map(|v| v.max()).unwrap_or(0) as i16,
        )
    }
    pub fn dam_pct_min(&self) -> Dam {
        let Some(ids) = self.identifications.as_ref() else {
            return Dam::default();
        };
        Dam::new(
            ids.neutral_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.earth_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.thunder_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.water_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.fire_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.air_damage.map(|v| v.min()).unwrap_or(0) as i16,
        )
    }
    pub fn common_stat_max(&self) -> CommonStat {
        let Some(ids) = self.identifications.as_ref() else {
            return CommonStat::default();
        };
        CommonStat::new(
            ids.health_regen_raw.map(|v| v.max()).unwrap_or(0) as i16,
            ids.health_regen.map(|v| v.max()).unwrap_or(0) as i16,
            ids.mana_regen.map(|v| v.max()).unwrap_or(0) as i16,
            ids.life_steal.map(|v| v.max()).unwrap_or(0) as i16,
            ids.mana_steal.map(|v| v.max()).unwrap_or(0) as i16,
            ids.walk_speed.map(|v| v.max()).unwrap_or(0) as i16,
            ids.raw_spell_damage.map(|v| v.max()).unwrap_or(0) as i16,
            ids.spell_damage.map(|v| v.max()).unwrap_or(0) as i16,
        )
    }
    pub fn common_stat_min(&self) -> CommonStat {
        let Some(ids) = self.identifications.as_ref() else {
            return CommonStat::default();
        };
        CommonStat::new(
            ids.health_regen_raw.map(|v| v.min()).unwrap_or(0) as i16,
            ids.health_regen.map(|v| v.min()).unwrap_or(0) as i16,
            ids.mana_regen.map(|v| v.min()).unwrap_or(0) as i16,
            ids.life_steal.map(|v| v.min()).unwrap_or(0) as i16,
            ids.mana_steal.map(|v| v.min()).unwrap_or(0) as i16,
            ids.walk_speed.map(|v| v.min()).unwrap_or(0) as i16,
            ids.raw_spell_damage.map(|v| v.min()).unwrap_or(0) as i16,
            ids.spell_damage.map(|v| v.min()).unwrap_or(0) as i16,
        )
    }
    pub fn sec_stat_max(&self) -> SecStat {
        let Some(ids) = self.identifications.as_ref() else {
            return SecStat::default();
        };
        SecStat::new(
            ids.xp_bonus.map(|v| v.max()).unwrap_or(0) as i16,
            ids.loot_bonus.map(|v| v.max()).unwrap_or(0) as i16,
        )
    }
    pub fn sec_stat_min(&self) -> SecStat {
        let Some(ids) = self.identifications.as_ref() else {
            return SecStat::default();
        };
        SecStat::new(
            ids.xp_bonus.map(|v| v.min()).unwrap_or(0) as i16,
            ids.loot_bonus.map(|v| v.min()).unwrap_or(0) as i16,
        )
    }
    pub fn damages(&self) -> Damages {
        let Some(base) = self.base.as_ref() else {
            return Damages::default();
        };
        Damages::new(
            base.base_damage.map(|v| v.min()).unwrap_or(0) as f64,
            base.base_earth_damage.map(|v| v.min()).unwrap_or(0) as f64,
            base.base_thunder_damage.map(|v| v.min()).unwrap_or(0) as f64,
            base.base_water_damage.map(|v| v.min()).unwrap_or(0) as f64,
            base.base_fire_damage.map(|v| v.min()).unwrap_or(0) as f64,
            base.base_air_damage.map(|v| v.min()).unwrap_or(0) as f64,
            base.base_damage.map(|v| v.max()).unwrap_or(0) as f64,
            base.base_earth_damage.map(|v| v.max()).unwrap_or(0) as f64,
            base.base_thunder_damage.map(|v| v.max()).unwrap_or(0) as f64,
            base.base_water_damage.map(|v| v.max()).unwrap_or(0) as f64,
            base.base_fire_damage.map(|v| v.max()).unwrap_or(0) as f64,
            base.base_air_damage.map(|v| v.max()).unwrap_or(0) as f64,
        )
    }
    pub fn damage_present(&self) -> Mask {
        let Some(base) = self.base.as_ref() else {
            return Mask::default();
        };
        Mask::from_slice([
            base.base_damage.is_some(),
            base.base_earth_damage.is_some(),
            base.base_thunder_damage.is_some(),
            base.base_water_damage.is_some(),
            base.base_fire_damage.is_some(),
            base.base_air_damage.is_some(),
        ])
    }
    pub fn attack_speed(&self) -> Option<AtkSpd> {
        self.attack_speed.clone().map(|v| v.into())
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
            ItemType::Material => write!(f, "Material"),
            ItemType::Tool => write!(f, "Tool"),
            ItemType::Ingredient => write!(f, "Ingredient"),
            ItemType::Charm => write!(f, "Charm"),
            ItemType::Tome => write!(f, "Tome"),
        }
    }
}
