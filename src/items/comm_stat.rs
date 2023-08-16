use std::{
    ops::Add,
    simd::{i16x8, SimdPartialEq, SimdPartialOrd},
};

use crate::*;

/// 0:hpr_raw 1:hpr_pct 2:mr 3:ls 4:ms 5:spd 6:sd_raw 7:sd_pct
#[derive(Clone, Debug, Default, Hash)]
pub struct CommonStat {
    pub inner: i16x8,
}
impl CommonStat {
    pub fn new(
        hpr_raw: i16,
        hpr_pct: i16,
        mr: i16,
        ls: i16,
        ms: i16,
        spd: i16,
        sd_raw: i16,
        sd_pct: i16,
    ) -> Self {
        CommonStat {
            inner: i16x8::from([hpr_raw, hpr_pct, mr, ls, ms, spd, sd_raw, sd_pct]),
        }
    }
    pub fn any_lt(&self, other: &Self) -> bool {
        self.inner.simd_lt(other.inner).any()
    }
    pub fn hpr_raw(&self) -> i16 {
        self.inner[0]
    }
    pub fn hpr_pct(&self) -> i16 {
        self.inner[1]
    }
    pub fn hpr(&self) -> i32 {
        (100 + self.hpr_pct() as i32) * self.hpr_raw() as i32 / 100
    }
    pub fn mr(&self) -> i16 {
        self.inner[2]
    }
    pub fn ls(&self) -> i16 {
        self.inner[3]
    }
    pub fn ms(&self) -> i16 {
        self.inner[4]
    }
    pub fn spd(&self) -> i16 {
        self.inner[5]
    }
    pub fn sd_raw(&self) -> i16 {
        self.inner[6]
    }
    pub fn sd_pct(&self) -> i16 {
        self.inner[7]
    }

    pub fn sum_max_stats(apparels: &[&Apparel], weapon: &Weapon) -> Self {
        let mut total: i16x8 = weapon.stat_max.inner;

        for item in apparels {
            total += item.stat_max.inner;
        }
        Self { inner: total }
    }
}
impl PartialEq for CommonStat {
    fn eq(&self, other: &Self) -> bool {
        self.inner.simd_eq(other.inner).all()
    }
}
impl Add for CommonStat {
    type Output = CommonStat;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            inner: self.inner + rhs.inner,
        }
    }
}
impl std::fmt::Display for CommonStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(mr:{}, ms:{}, spd:{}, ls:{}, hpr_raw:{}, hpr_pct:{}, sd_raw:{}, sd_pct:{})",
            self.mr(),
            self.ms(),
            self.spd(),
            self.ls(),
            self.hpr_raw(),
            self.hpr_pct(),
            self.sd_raw(),
            self.sd_pct(),
        )
    }
}
#[cfg(test)]
mod tests {
    use crate::gen_test_apparels;

    use super::*;

    #[test]
    fn sum_max_stats_works() {
        let apparels = gen_test_apparels();
        let apparels: Vec<&Apparel> = apparels.iter().map(|v| v).collect();
        assert_eq!(
            CommonStat::sum_max_stats(apparels.as_slice(), &Default::default()),
            CommonStat::new(130, 30, 77, 214, -8, 16, 621, 77)
        )
    }
}
