use std::ops::AddAssign;
use std::simd::cmp::SimdPartialEq;
use std::simd::cmp::SimdPartialOrd;
use std::{ops::Add, simd::i16x8};

use crate::calculate::*;

use super::*;

/// 0:hpr_raw 1:hpr_pct 2:mr 3:ls 4:ms 5:spd 6:sd_raw 7:sd_pct
#[derive(Clone, Debug, Default)]
pub struct CommonStat {
    pub inner: i16x8,
}

impl AddAssign<&CommonStat> for CommonStat {
    fn add_assign(&mut self, rhs: &Self) {
        self.inner += rhs.inner;
    }
}

impl CommonStat {
    #[allow(clippy::too_many_arguments)]
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
    // https://github.com/hppeng-wynn/hppeng-wynn.github.io/blob/f01c29a099ee21ed57bed9054b4651a311ee40cd/js/builder/builder_graph.js#L528
    pub fn hpr(&self) -> i32 {
        self.hpr_raw() as i32 + self.hpr_pct() as i32 * self.hpr_raw().abs() as i32 / 100
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
        let mut total: i16x8 = weapon.common_stat_max.inner;

        for item in apparels {
            total += item.common_stat_max.inner;
        }
        Self { inner: total }
    }
}
impl Roll for CommonStat {
    fn min_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: min_rolls(&self.inner),
            }
        }
    }

    fn max_roll(&self, fix_id: bool) -> Self {
        if fix_id {
            self.clone()
        } else {
            Self {
                inner: max_rolls(&self.inner),
            }
        }
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
impl Add<&CommonStat> for &CommonStat {
    type Output = CommonStat;

    fn add(self, rhs: &CommonStat) -> Self::Output {
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
#[derive(serde::Deserialize)]
struct CommonStatJson {
    hpr_raw: i16,
    hpr_pct: i16,
    mr: i16,
    ls: i16,
    ms: i16,
    spd: i16,
    sd_raw: i16,
    sd_pct: i16,
}
impl<'de> serde::Deserialize<'de> for CommonStat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json_representation = CommonStatJson::deserialize(deserializer)?;
        Ok(CommonStat::new(
            json_representation.hpr_raw,
            json_representation.hpr_pct,
            json_representation.mr,
            json_representation.ls,
            json_representation.ms,
            json_representation.spd,
            json_representation.sd_raw,
            json_representation.sd_pct,
        ))
    }
}
#[cfg(test)]
mod tests {
    use crate::tests::*;

    use super::*;

    #[test]
    fn sum_max_stats_works() {
        let apparels = gen_test_apparels();
        for v in apparels {
            let apparels: [&Apparel; 8] = v.apparels.iter().collect::<Vec<_>>().try_into().unwrap();
            assert_eq!(
                CommonStat::sum_max_stats(apparels.as_slice(), &Default::default()),
                v.common_stat
            )
        }
    }

    #[test]
    fn hpr_works() {
        assert_eq!(CommonStat::new(100, 10, 0, 0, 0, 0, 0, 0).hpr(), 110);
        assert_eq!(CommonStat::new(100, -10, 0, 0, 0, 0, 0, 0).hpr(), 90);
        assert_eq!(CommonStat::new(-100, -10, 0, 0, 0, 0, 0, 0).hpr(), -110);
    }
}
