use std::simd::cmp::SimdPartialOrd;
use std::simd::num::SimdInt;
use std::{
    fmt,
    ops::{Add, AddAssign, Sub},
    simd::i16x8,
};
/// 0:e 1:t 2:w 3:f 4:a
#[derive(Clone, Debug, PartialEq, Default, Hash)]
pub struct Point {
    pub inner: i16x8,
}
impl Point {
    pub fn new(e: i16, t: i16, w: i16, f: i16, a: i16) -> Self {
        Self {
            inner: i16x8::from_slice(&[e, t, w, f, a, 0, 0, 0]),
        }
    }
    pub fn splat(value: i16) -> Self {
        Self {
            inner: i16x8::from_slice(&[value, value, value, value, value, 0, 0, 0]),
        }
    }
    pub fn any_lt(&self, other: &Self) -> bool {
        self.inner.simd_lt(other.inner).any()
    }
    pub fn merge_max(&self, other: &Self) -> Self {
        let mask = self.inner.simd_lt(other.inner);
        Self {
            inner: mask.select(other.inner, self.inner),
        }
    }
    pub fn e(&self) -> i16 {
        self.inner[0]
    }
    pub fn t(&self) -> i16 {
        self.inner[1]
    }
    pub fn w(&self) -> i16 {
        self.inner[2]
    }
    pub fn f(&self) -> i16 {
        self.inner[3]
    }
    pub fn a(&self) -> i16 {
        self.inner[4]
    }
    pub fn sum(&self) -> i16 {
        self.inner.reduce_sum()
    }
    pub fn only_negative(&self) -> Point {
        let zero = i16x8::splat(0);
        // mask = data < 0
        let mask = self.inner.simd_lt(zero);
        Point {
            inner: mask.select(self.inner, zero),
        }
    }
}
impl AddAssign<&Point> for Point {
    fn add_assign(&mut self, rhs: &Point) {
        self.inner += rhs.inner;
    }
}
impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self::Output {
            inner: self.inner + rhs.inner,
        }
    }
}
impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::Output {
            inner: self.inner - rhs.inner,
        }
    }
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "earth:{}\tthunder:{}\twater:{}\tfire:{}\tair:{}",
            self.e(),
            self.t(),
            self.w(),
            self.f(),
            self.a()
        )
    }
}
impl From<i16x8> for Point {
    fn from(value: i16x8) -> Self {
        Self { inner: value }
    }
}
#[derive(serde::Deserialize)]
struct PointJson {
    e: i16,
    t: i16,
    w: i16,
    f: i16,
    a: i16,
}
impl<'de> serde::Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let json_representation = PointJson::deserialize(deserializer)?;
        Ok(Point::new(
            json_representation.e,
            json_representation.t,
            json_representation.w,
            json_representation.f,
            json_representation.a,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merge_max_works() {
        let a = Point::new(1, 1, 1, 0, 0);
        let b = Point::new(0, 0, 0, 1, 1);
        assert_eq!(a.merge_max(&b), Point::new(1, 1, 1, 1, 1));

        let a = Point::new(1, -1, 1, 1, 1);
        let b = Point::new(1, 0, 0, 2, 1);
        assert_eq!(a.merge_max(&b), Point::new(1, 0, 1, 2, 1));
    }
}
