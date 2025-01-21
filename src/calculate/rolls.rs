use std::simd::cmp::SimdPartialOrd;
use std::simd::num::SimdFloat;
use std::simd::num::SimdInt;
use std::simd::{f32x8, i16x8};

pub trait Roll {
    fn min_roll(&self, fix_id: bool) -> Self;
    fn max_roll(&self, fix_id: bool) -> Self;
}

impl Roll for i32 {
    fn min_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            *self
        } else {
            min_rolls_i32(*self)
        }
    }

    fn max_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            *self
        } else {
            max_rolls_i32(*self)
        }
    }
}

pub fn max_roll<T: Roll>(value: &T, fix_id: bool) -> T {
    value.max_roll(fix_id)
}
pub fn min_roll<T: Roll>(value: &T, fix_id: bool) -> T {
    value.min_roll(fix_id)
}
// https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/befbc208ad760d455d5966cbec9c70ed65680f0e/js/build_utils.js#L202-L235
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

/// Adjusts the values in a SIMD vector based on positive and negative multipliers.
///
/// This function takes a SIMD vector of 8 `i16` values and adjusts each value based on whether
/// it is positive or negative. Positive values are multiplied by the `pos` multiplier, and
/// negative values are multiplied by the `neg` multiplier. The results are then rounded to the
/// nearest integer and combined back into a single SIMD vector.
///
/// # Parameters
///
/// - `stat`: A reference to a SIMD vector of 8 `i16` values to be adjusted.
/// - `pos`: A `f32` multiplier applied to positive values in the `stat` vector.
/// - `neg`: A `f32` multiplier applied to negative values in the `stat` vector.
///
/// # Returns
///
/// A SIMD vector of 8 `i16` values where each value has been adjusted based on its sign and the
/// corresponding multiplier.
///
/// # Example
///
/// ```rust
/// use std::simd::{i16x8, f32x8};
///
/// let stat = i16x8::from([-10, 0, 10, 2, -5, 3, -1, 7]);
/// let pos_multiplier = 0.7;
/// let neg_multiplier = 1.3;
/// let result = rolls(&stat, pos_multiplier, neg_multiplier);
/// assert_eq!(result.to_array(), [-13, 0, 7, 1, -7, 2, -1, 5]);
/// ```
///
/// # Explanation
///
/// 1. **Initialization**:
///    - `zero`: A SIMD vector of 8 zeros.
///    - `mask`: A mask indicating which elements in `stat` are greater than zero.
///
/// 2. **Separation**:
///    - `positive`: A SIMD vector containing the positive values from `stat`.
///    - `negative`: A SIMD vector containing the negative values from `stat`.
///
/// 3. **Adjustment**:
///    - `positive`: Positive values are multiplied by `pos` and rounded.
///    - `negative`: Negative values are multiplied by `neg` and rounded.
///
/// 4. **Combination**:
///    - The adjusted positive and negative values are combined back into a single SIMD vector.
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
