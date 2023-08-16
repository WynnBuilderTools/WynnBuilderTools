use std::str::FromStr;

#[derive(Clone, Debug, Default)]
pub enum Class {
    #[default]
    Relik,
    Bow,
    Wand,
    Dagger,
    Spear,
}

impl Class {
    pub fn class_def_mult(&self) -> f64 {
        match &self {
            Class::Relik => 0.60,
            Class::Bow => 0.70,
            Class::Wand => 0.80,
            Class::Dagger => 1.0,
            Class::Spear => 1.0,
        }
    }
}
impl FromStr for Class {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "relik" => Ok(Class::Relik),
            "bow" => Ok(Class::Bow),
            "wand" => Ok(Class::Wand),
            "dagger" => Ok(Class::Dagger),
            "spear" => Ok(Class::Spear),
            _ => Err(format!("Invalid value for class, found '{}'", s)),
        }
    }
}
