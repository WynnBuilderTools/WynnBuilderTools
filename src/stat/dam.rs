use crate::*;

pub fn sum_dam_pct_max(apparels: &[&Apparel], weapon: &Weapon) -> Dam {
    let mut total: Dam = weapon.dam_pct_max.clone();
    for item in apparels {
        total += &item.dam_pct_max;
    }
    total
}
