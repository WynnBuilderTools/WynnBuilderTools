use std::str::FromStr;

use super::*;

#[derive(Clone, Debug, Default)]
pub enum AtkSpd {
    #[default]
    SuperSlow,
    VerySlow,
    Slow,
    Normal,
    Fast,
    VeryFast,
    SuperFast,
}

impl AtkSpd {
    pub fn speed_mult(&self) -> f64 {
        match *self {
            AtkSpd::SuperSlow => 0.51,
            AtkSpd::VerySlow => 0.83,
            AtkSpd::Slow => 1.5,
            AtkSpd::Normal => 2.05,
            AtkSpd::Fast => 2.5,
            AtkSpd::VeryFast => 3.1,
            AtkSpd::SuperFast => 4.3,
        }
    }
}

impl From<AttackSpeed> for AtkSpd {
    fn from(value: AttackSpeed) -> Self {
        match value {
            AttackSpeed::Fast => AtkSpd::Fast,
            AttackSpeed::Normal => AtkSpd::Normal,
            AttackSpeed::Slow => AtkSpd::Slow,
            AttackSpeed::SuperFast => AtkSpd::SuperFast,
            AttackSpeed::SuperSlow => AtkSpd::SuperSlow,
            AttackSpeed::VeryFast => AtkSpd::VeryFast,
            AttackSpeed::VerySlow => AtkSpd::VerySlow,
        }
    }
}

impl FromStr for AtkSpd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SUPER_SLOW" => Ok(AtkSpd::SuperSlow),
            "VERY_SLOW" => Ok(AtkSpd::VerySlow),
            "SLOW" => Ok(AtkSpd::Slow),
            "NORMAL" => Ok(AtkSpd::Normal),
            "FAST" => Ok(AtkSpd::Fast),
            "VERY_FAST" => Ok(AtkSpd::VeryFast),
            "SUPER_FAST" => Ok(AtkSpd::SuperFast),
            _ => Err(format!("Invalid value for atk spd, found '{}'", s)),
        }
    }
}
