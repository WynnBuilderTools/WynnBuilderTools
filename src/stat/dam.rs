use crate::*;
use std::simd::{f64x8, i16x8, SimdInt};

pub fn sum_dam_pct_max(value: &[&Apparel], weapon: &Weapon) -> [f64; 6] {
    let mut total: i16x8 = weapon.dam_pct_max.inner.clone();
    for item in value {
        total += &item.dam_pct_max.inner;
    }
    let result: f64x8 = total.cast() / f64x8::splat(100.0);
    [
        result[0], result[1], result[2], result[3], result[4], result[5],
    ]
}
