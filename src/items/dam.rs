use std::{ops::AddAssign, simd::i16x8};

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
}
impl AddAssign<&Dam> for Dam {
    fn add_assign(&mut self, rhs: &Dam) {
        self.inner += rhs.inner;
    }
}
