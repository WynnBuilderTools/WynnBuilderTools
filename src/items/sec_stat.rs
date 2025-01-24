use std::simd::{cmp::SimdPartialOrd, i16x8};

use super::*;

#[derive(Clone, Debug, Default)]
pub struct SecStat {
    pub inner: i16x8,
}

impl SecStat {
    pub fn new(exp_bonus: i16, loot_bonus: i16) -> Self {
        Self {
            inner: i16x8::from([exp_bonus, loot_bonus, 0, 0, 0, 0, 0, 0]),
        }
    }
    pub fn sum_max_stats(apparels: &[&Apparel], weapon: &Weapon) -> Self {
        let mut total: i16x8 = weapon.common_stat_max.inner;

        for item in apparels {
            total += item.sec_stat_max.inner;
        }
        Self { inner: total }
    }
    pub fn exp_bonus(&self) -> i16 {
        self.inner[0]
    }
    pub fn loot_bonus(&self) -> i16 {
        self.inner[1]
    }
    pub fn any_lt(&self, other: &Self) -> bool {
        self.inner.simd_lt(other.inner).any()
    }
}

impl std::fmt::Display for SecStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(exp_bonus:{}, loot_bonus:{})",
            self.exp_bonus(),
            self.loot_bonus(),
        )
    }
}
