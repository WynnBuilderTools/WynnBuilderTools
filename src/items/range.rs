use std::{
    ops::{AddAssign, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Clone, Debug, Default)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}
impl Range {
    pub fn new(min: f64, max: f64) -> Range {
        Range { min, max }
    }
    pub fn avg(&self) -> f64 {
        (self.min + self.max) / 2.0
    }
}

impl FromStr for Range {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = match s.split_once("-") {
            Some((min, max)) => (
                min.parse()
                    .map_err(|e| format!("Failed to parse min value: {}", e))?,
                max.parse()
                    .map_err(|e| format!("Failed to parse max value: {}", e))?,
            ),
            None => {
                return Err(format!(
                    "Invalid value for range: expected format 'min-max', found '{}'",
                    s
                ));
            }
        };
        Ok(Range { min, max })
    }
}
impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        const THRESHOLD: f64 = 0.01;
        (self.min - other.min).abs() < THRESHOLD && (self.max - other.max).abs() < THRESHOLD
    }
}
impl AddAssign<&Range> for Range {
    fn add_assign(&mut self, rhs: &Range) {
        self.min += rhs.min;
        self.max += rhs.max;
    }
}
impl Mul<f64> for &Range {
    type Output = Range;

    fn mul(self, rhs: f64) -> Self::Output {
        Range {
            min: self.min * rhs,
            max: self.max * rhs,
        }
    }
}
impl std::ops::MulAssign<f64> for Range {
    fn mul_assign(&mut self, rhs: f64) {
        self.min *= rhs;
        self.max *= rhs;
    }
}
impl Sub<&Range> for &Range {
    type Output = Range;

    fn sub(self, rhs: &Range) -> Self::Output {
        Range {
            min: self.min - rhs.min,
            max: self.max - rhs.max,
        }
    }
}
impl Div<&Range> for &Range {
    type Output = Range;

    fn div(self, rhs: &Range) -> Self::Output {
        Range {
            min: self.min / rhs.min,
            max: self.max / rhs.max,
        }
    }
}
