mod ability;
mod dam;
mod damage_calculate;
mod def;
mod hp;
mod hppeng;
mod rolls;
mod skill_point;

pub use ability::*;
pub use dam::*;
pub use damage_calculate::*;
pub use def::*;
pub use hp::*;
pub use hppeng::*;
pub use rolls::*;
pub use skill_point::*;


/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/HEAD/js/build_utils.js#L8
pub fn skill_points_to_percentage(skp: i16) -> f64 {
    let mut skp = skp;
    if skp <= 0 {
        return 0.0;
    } else if skp >= 150 {
        skp = 150;
    }
    const R: f64 = 0.9908;
    (R / (1.0 - R) * (1.0 - R.powi(skp as i32))) / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_points_to_percentage_works() {
        assert_eq!(skill_points_to_percentage(50), 0.3985374219798806);
    }
}
