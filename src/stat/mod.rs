mod dam;
mod damage_calculate;
mod def;
mod hp;
mod hppeng;
mod rolls;
mod skill_point;

pub use dam::*;
pub use def::*;
pub use hp::*;
pub use hppeng::*;
pub use rolls::*;
pub use skill_point::*;

use crate::*;
use std::fs;

pub fn gen_test_apparels() -> [Apparel; 8] {
    let file = fs::read("assets/apparels.json").unwrap();
    let items: [Item; 8] = serde_json::from_slice(&file).unwrap();
    items.map(|v| Apparel::try_from(&v).unwrap())
}

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/HEAD/js/build_utils.js#L8
pub fn skill_points_to_percentage(skp: i32) -> f64 {
    let mut skp = skp;
    if skp <= 0 {
        return 0.0;
    } else if skp >= 150 {
        skp = 150;
    }
    const R: f64 = 0.9908;
    (R / (1.0 - R) * (1.0 - R.powi(skp))) / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_points_to_percentage_works() {
        assert_eq!(skill_points_to_percentage(50), 0.3985374219798806);
    }
}
