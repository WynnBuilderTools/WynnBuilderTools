use crate::*;

pub fn sum_hp_max(items: &[&Apparel], weapon: &Weapon) -> i32 {
    let mut hp = weapon.hp_bonus_max;
    for v in items {
        hp += v.hp;
        hp += v.hp_bonus_max;
    }
    hp
}
/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/HEAD/js/builder/builder_graph.js#L528
pub fn ehp(point: &SkillPoints, hp: i32, class: &Class) -> i32 {
    let def_pct = skill_points_to_percentage(point.original.f()) * 0.867;
    let agi_pct = skill_points_to_percentage(point.original.a()) * 0.951;
    let base_ehp = hp as f64 / (0.1 * agi_pct + (1.0 - agi_pct) * (1.0 - def_pct));
    // TODO: add skill effect
    (base_ehp / (2.0 - class.def_mult())) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ehp_works() {
        assert_eq!(
            ehp(
                &SkillPoints {
                    assign: Default::default(),
                    original: Point::new(0, 0, 0, 50, 25),
                },
                535,
                &Class::Mage,
            ),
            829
        );
    }
}
