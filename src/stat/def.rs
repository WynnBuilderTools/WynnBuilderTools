use std::simd::i16x8;

use crate::*;

pub fn sum_def_max(value: &[&Apparel], weapon: &Weapon) -> Point {
    let mut def_total: Point = Default::default();
    let mut def_pct_total: Point = weapon.def_pct_max.clone();
    for item in value {
        def_total += &item.def;
        def_pct_total += &item.def_pct_max;
    }
    Point {
        inner: def_total.inner * (i16x8::splat(100) + def_pct_total.inner) / i16x8::splat(100),
    }
}
