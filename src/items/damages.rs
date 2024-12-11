use crate::*;

#[derive(Debug, Default, Clone)]
pub struct Damages {
    inner: [Range; 6],
}
impl Damages {
    pub fn from_slice(damages: [Range; 6]) -> Self {
        Self { inner: damages }
    }
    pub fn new(
        n_min: f64,
        e_min: f64,
        t_min: f64,
        w_min: f64,
        f_min: f64,
        a_min: f64,
        n_max: f64,
        e_max: f64,
        t_max: f64,
        w_max: f64,
        f_max: f64,
        a_max: f64,
    ) -> Self {
        Self {
            inner: [
                Range::new(n_min, n_max),
                Range::new(e_min, e_max),
                Range::new(t_min, t_max),
                Range::new(w_min, w_max),
                Range::new(f_min, f_max),
                Range::new(a_min, a_max),
            ],
        }
    }
    pub fn splat(damage: &Range) -> Self {
        Self::from_slice([
            damage.clone(),
            damage.clone(),
            damage.clone(),
            damage.clone(),
            damage.clone(),
            damage.clone(),
        ])
    }
    pub fn total(&self) -> Range {
        let (min_sum, max_sum) = self.inner.iter().fold((0.0, 0.0), |(min_acc, max_acc), v| {
            (min_acc + v.min, max_acc + v.max)
        });

        Range {
            min: min_sum,
            max: max_sum,
        }
    }
    pub fn only_rainbow(&self) -> Self {
        Self::from_slice([
            Range { min: 0.0, max: 0.0 },
            self.inner[1].clone(),
            self.inner[2].clone(),
            self.inner[3].clone(),
            self.inner[4].clone(),
            self.inner[5].clone(),
        ])
    }
}
impl From<&DamagesConvert> for Damages {
    fn from(value: &DamagesConvert) -> Self {
        Self::from_slice([
            Range {
                min: value.inner[0],
                max: value.inner[0],
            },
            Range {
                min: value.inner[1],
                max: value.inner[1],
            },
            Range {
                min: value.inner[2],
                max: value.inner[2],
            },
            Range {
                min: value.inner[3],
                max: value.inner[3],
            },
            Range {
                min: value.inner[4],
                max: value.inner[4],
            },
            Range {
                min: value.inner[5],
                max: value.inner[5],
            },
        ])
    }
}
impl std::ops::AddAssign<&Damages> for Damages {
    fn add_assign(&mut self, rhs: &Damages) {
        self.inner[0] += &rhs.inner[0];
        self.inner[1] += &rhs.inner[1];
        self.inner[2] += &rhs.inner[2];
        self.inner[3] += &rhs.inner[3];
        self.inner[4] += &rhs.inner[4];
        self.inner[5] += &rhs.inner[5];
    }
}
impl std::ops::Div<&Range> for &Damages {
    type Output = Damages;
    fn div(self, rhs: &Range) -> Self::Output {
        Damages::from_slice([
            &self.inner[0] / rhs,
            &self.inner[1] / rhs,
            &self.inner[2] / rhs,
            &self.inner[3] / rhs,
            &self.inner[4] / rhs,
            &self.inner[5] / rhs,
        ])
    }
}
impl std::ops::Mul<f64> for &Damages {
    type Output = Damages;
    fn mul(self, rhs: f64) -> Self::Output {
        Damages::from_slice([
            &self.inner[0] * rhs,
            &self.inner[1] * rhs,
            &self.inner[2] * rhs,
            &self.inner[3] * rhs,
            &self.inner[4] * rhs,
            &self.inner[5] * rhs,
        ])
    }
}
impl std::ops::Mul<&DamagesConvert> for &Damages {
    type Output = Damages;
    fn mul(self, rhs: &DamagesConvert) -> Self::Output {
        Damages::from_slice([
            &self.inner[0] * rhs.inner[0],
            &self.inner[1] * rhs.inner[1],
            &self.inner[2] * rhs.inner[2],
            &self.inner[3] * rhs.inner[3],
            &self.inner[4] * rhs.inner[4],
            &self.inner[5] * rhs.inner[5],
        ])
    }
}
impl std::ops::MulAssign<f64> for Damages {
    fn mul_assign(&mut self, rhs: f64) {
        self.inner[0] *= rhs;
        self.inner[1] *= rhs;
        self.inner[2] *= rhs;
        self.inner[3] *= rhs;
        self.inner[4] *= rhs;
        self.inner[5] *= rhs;
    }
}
impl std::ops::MulAssign<&DamagesConvert> for Damages {
    fn mul_assign(&mut self, rhs: &DamagesConvert) {
        self.inner[0] *= rhs.inner[0];
        self.inner[1] *= rhs.inner[1];
        self.inner[2] *= rhs.inner[2];
        self.inner[3] *= rhs.inner[3];
        self.inner[4] *= rhs.inner[4];
        self.inner[5] *= rhs.inner[5];
    }
}
impl PartialEq for Damages {
    fn eq(&self, other: &Self) -> bool {
        self.inner[0] == other.inner[0]
            && self.inner[1] == other.inner[1]
            && self.inner[2] == other.inner[2]
            && self.inner[3] == other.inner[3]
            && self.inner[4] == other.inner[4]
            && self.inner[5] == other.inner[5]
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DamagesConvert {
    inner: [f64; 6],
}
impl From<&Dam> for DamagesConvert {
    fn from(value: &Dam) -> Self {
        Self::from_slice([
            value.inner[0] as f64 / 100.0,
            value.inner[1] as f64 / 100.0,
            value.inner[2] as f64 / 100.0,
            value.inner[3] as f64 / 100.0,
            value.inner[4] as f64 / 100.0,
            value.inner[5] as f64 / 100.0,
        ])
    }
}
impl DamagesConvert {
    /// n e t w f a
    pub fn from_slice(items: [f64; 6]) -> Self {
        Self { inner: items }
    }
    pub fn from_slice_i32(items: &[i32; 6]) -> Self {
        Self {
            inner: [
                (items[0] as f64) / 100.0,
                (items[1] as f64) / 100.0,
                (items[2] as f64) / 100.0,
                (items[3] as f64) / 100.0,
                (items[4] as f64) / 100.0,
                (items[5] as f64) / 100.0,
            ],
        }
    }
    pub fn splat(item: f64) -> Self {
        Self::from_slice([item, item, item, item, item, item])
    }
    pub fn only_rainbow(&self) -> Self {
        Self::from_slice([
            0.0,
            self.inner[1],
            self.inner[2],
            self.inner[3],
            self.inner[4],
            self.inner[5],
        ])
    }
    pub fn gt(&self, value: f64) -> Mask {
        Mask::from_slice([
            self.inner[0] > value,
            self.inner[1] > value,
            self.inner[2] > value,
            self.inner[3] > value,
            self.inner[4] > value,
            self.inner[5] > value,
        ])
    }
    pub fn neutral(&self) -> f64 {
        self.inner[0]
    }
    pub fn e(&self) -> f64 {
        self.inner[1]
    }
    pub fn t(&self) -> f64 {
        self.inner[2]
    }
    pub fn w(&self) -> f64 {
        self.inner[3]
    }
    pub fn f(&self) -> f64 {
        self.inner[4]
    }
    pub fn a(&self) -> f64 {
        self.inner[5]
    }
    pub fn only_positive(&self) -> Self {
        DamagesConvert::from_slice(self.inner.map(|v| num::Float::max(v, 0.0)))
    }
    pub fn total(&self) -> f64 {
        self.inner.iter().fold(0.0, |acc, x| acc + x)
    }
}

impl std::ops::AddAssign<&DamagesConvert> for DamagesConvert {
    fn add_assign(&mut self, rhs: &DamagesConvert) {
        self.inner[0] += &rhs.inner[0];
        self.inner[1] += &rhs.inner[1];
        self.inner[2] += &rhs.inner[2];
        self.inner[3] += &rhs.inner[3];
        self.inner[4] += &rhs.inner[4];
        self.inner[5] += &rhs.inner[5];
    }
}
impl std::ops::AddAssign<f64> for DamagesConvert {
    fn add_assign(&mut self, rhs: f64) {
        self.inner[0] += rhs;
        self.inner[1] += rhs;
        self.inner[2] += rhs;
        self.inner[3] += rhs;
        self.inner[4] += rhs;
        self.inner[5] += rhs;
    }
}
impl std::ops::Mul<&DamagesConvert> for &DamagesConvert {
    type Output = DamagesConvert;

    fn mul(self, rhs: &DamagesConvert) -> Self::Output {
        DamagesConvert::from_slice([
            self.inner[0] * rhs.inner[0],
            self.inner[1] * rhs.inner[1],
            self.inner[2] * rhs.inner[2],
            self.inner[3] * rhs.inner[3],
            self.inner[4] * rhs.inner[4],
            self.inner[5] * rhs.inner[5],
        ])
    }
}

#[derive(Debug, Default, Clone)]
pub struct Mask {
    inner: [bool; 6],
}
impl Mask {
    pub fn from_slice(items: [bool; 6]) -> Self {
        Self { inner: items }
    }
    pub fn splat(item: bool) -> Self {
        Self::from_slice([item, item, item, item, item, item])
    }
}
impl Select<&Damages> for Mask {
    type Output = Damages;
    fn select(&self, item: &Damages) -> Self::Output {
        let mut result = item.clone();
        for i in 0..6 {
            if !self.inner[i] {
                result.inner[i] = Default::default();
            }
        }
        result
    }
}
impl Select<&DamagesConvert> for Mask {
    type Output = DamagesConvert;

    fn select(&self, item: &DamagesConvert) -> Self::Output {
        let mut result = item.clone();
        for i in 0..6 {
            if !self.inner[i] {
                result.inner[i] = Default::default();
            }
        }
        result
    }
}
pub trait Select<T> {
    type Output;
    fn select(&self, item: T) -> Self::Output;
}

impl std::ops::BitOr for Mask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self::from_slice([
            self.inner[0] | rhs.inner[0],
            self.inner[1] | rhs.inner[1],
            self.inner[2] | rhs.inner[2],
            self.inner[3] | rhs.inner[3],
            self.inner[4] | rhs.inner[4],
            self.inner[5] | rhs.inner[5],
        ])
    }
}
