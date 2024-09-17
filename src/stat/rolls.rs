use std::simd::cmp::SimdPartialOrd;
use std::simd::num::SimdFloat;
use std::simd::num::SimdInt;
use std::simd::{f32x8, i16x8};

/// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/HEAD/js/build_utils.js#L187
pub fn max_rolls(stat: &i16x8) -> i16x8 {
    // positive is 1.3,negative is 0.7
    rolls(stat, 1.3, 0.7)
}
pub fn min_rolls(stat: &i16x8) -> i16x8 {
    // positive is 0.3,negative is 1.3
    rolls(stat, 0.3, 1.3)
}
pub fn max_rolls_i32(stat: i32) -> i32 {
    // positive is 1.3,negative is 0.7
    if stat > 0 {
        stat as f64 * 1.3
    } else {
        stat as f64 * 0.7
    }
    .round() as i32
}
pub fn min_rolls_i32(stat: i32) -> i32 {
    // positive is 0.3,negative is 1.3
    if stat > 0 {
        stat as f64 * 0.3
    } else {
        stat as f64 * 1.3
    }
    .round() as i32
}

fn rolls(stat: &i16x8, pos: f32, neg: f32) -> i16x8 {
    let zero = i16x8::splat(0);
    let mask = stat.simd_gt(zero);
    let positive = mask.select(*stat, zero);
    let negative = mask.select(zero, *stat);

    let round_number = f32x8::splat(0.5);
    let positive = (positive.cast::<f32>() * f32x8::splat(pos) + round_number).cast::<i16>();
    let negative = (negative.cast::<f32>() * f32x8::splat(neg) - round_number).cast::<i16>();

    positive + negative
}

#[cfg(test)]
mod tests {
    use std::simd::i16x8;

    use super::*;

    #[test]
    fn max_rolls_works() {
        assert_eq!(
            max_rolls(&i16x8::from([-10, 0, 10, 2, 0, 0, 0, 0]),).to_array(),
            [-7, 0, 13, 3, 0, 0, 0, 0]
        );
    }
}
