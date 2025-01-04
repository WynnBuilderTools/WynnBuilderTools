use std::str::FromStr;

use serde::{de, Deserialize};

use super::*;

#[derive(Clone, Debug, Eq, PartialEq, Default, Hash)]
pub enum Class {
    #[default]
    Mage,
    Warrior,
    Archer,
    Assassin,
    Shaman,
}

impl Class {
    pub fn def_multi(&self) -> f64 {
        match &self {
            Class::Shaman => 0.60,
            Class::Archer => 0.70,
            Class::Mage => 0.80,
            Class::Assassin => 1.0,
            Class::Warrior => 1.0,
        }
    }
    pub fn weapon_type(&self) -> WeaponTypes {
        match &self {
            Class::Shaman => WeaponTypes::Relik,
            Class::Archer => WeaponTypes::Bow,
            Class::Mage => WeaponTypes::Wand,
            Class::Assassin => WeaponTypes::Dagger,
            Class::Warrior => WeaponTypes::Spear,
        }
    }
}
impl From<&Weapon> for Class {
    fn from(value: &Weapon) -> Self {
        match value.r#type {
            WeaponTypes::Relik => Class::Shaman,
            WeaponTypes::Bow => Class::Archer,
            WeaponTypes::Wand => Class::Mage,
            WeaponTypes::Dagger => Class::Assassin,
            WeaponTypes::Spear => Class::Warrior,
        }
    }
}
impl FromStr for Class {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Shaman" => Ok(Class::Shaman),
            "Archer" => Ok(Class::Archer),
            "Mage" => Ok(Class::Mage),
            "Assassin" => Ok(Class::Assassin),
            "Warrior" => Ok(Class::Warrior),
            _ => Err(format!("Invalid value for class, found '{}'", s)),
        }
    }
}
impl<'de> Deserialize<'de> for Class {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match Class::from_str(&s) {
            Ok(v) => Ok(v),
            Err(err) => Err(de::Error::custom(err)),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum WeaponTypes {
    #[default]
    Relik,
    Bow,
    Wand,
    Dagger,
    Spear,
}

impl FromStr for WeaponTypes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "relik" => Ok(WeaponTypes::Relik),
            "bow" => Ok(WeaponTypes::Bow),
            "wand" => Ok(WeaponTypes::Wand),
            "dagger" => Ok(WeaponTypes::Dagger),
            "spear" => Ok(WeaponTypes::Spear),
            _ => Err(format!("Invalid value for weapon type, found '{}'", s)),
        }
    }
}
