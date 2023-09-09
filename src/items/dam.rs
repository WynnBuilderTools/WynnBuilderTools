use std::{
    ops::AddAssign,
    simd::{i16x8, SimdPartialOrd},
};

#[derive(Clone, Debug, Default)]
pub struct Dam {
    pub inner: i16x8,
}
impl Dam {
    pub fn new(n: i16, e: i16, t: i16, w: i16, f: i16, a: i16) -> Self {
        Self {
            inner: i16x8::from_slice(&[n, e, t, w, f, a, 0, 0]),
        }
    }
    pub fn any_lt(&self, other: &Self) -> bool {
        self.inner.simd_lt(other.inner).any()
    }
    pub fn n(&self) -> i16 {
        self.inner[0]
    }
    pub fn e(&self) -> i16 {
        self.inner[1]
    }
    pub fn t(&self) -> i16 {
        self.inner[2]
    }
    pub fn w(&self) -> i16 {
        self.inner[3]
    }
    pub fn f(&self) -> i16 {
        self.inner[4]
    }
    pub fn a(&self) -> i16 {
        self.inner[5]
    }
}
impl AddAssign<&Dam> for Dam {
    fn add_assign(&mut self, rhs: &Dam) {
        self.inner += rhs.inner;
    }
}
impl std::fmt::Display for Dam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "earth:{}\tthunder:{}\twater:{}\tfire:{}\tair:{}\tneutral:{}",
            self.e(),
            self.t(),
            self.w(),
            self.f(),
            self.a(),
            self.n(),
        )
    }
}
