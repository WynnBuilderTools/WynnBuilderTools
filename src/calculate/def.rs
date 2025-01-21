use std::simd::{i16x8, num::SimdInt};

use crate::items::*;

// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/f01c29a099ee21ed57bed9054b4651a311ee40cd/js/builder/builder_graph.js#L541
pub fn sum_def_max(value: &[&Apparel], weapon: &Weapon) -> Point {
    let mut def_total: Point = Default::default();
    let mut def_pct_total: Point = weapon.def_pct_max.clone();
    for item in value {
        def_total += &item.def;
        def_pct_total += &item.def_pct_max;
    }
    Point {
        inner: def_total.inner + (def_pct_total.inner * def_total.inner.abs()) / i16x8::splat(100),
    }
}
