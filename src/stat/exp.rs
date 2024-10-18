use crate::*;

pub fn sum_exp_bonus_max(apparels: &[&Apparel], weapon: &Weapon) -> i32 {
    let mut total = weapon.max_exp_bonus;
    for item in apparels {
        total += item.max_exp_bonus;
    }
    total
}
