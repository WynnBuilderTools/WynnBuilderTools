use std::str::FromStr;

#[derive(Clone, Debug, Default)]
pub struct Range {
    pub min: f64,
    pub max: f64,
}
impl Range {
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
        self.min.floor() == other.min.floor() && self.max.floor() == other.max.floor()
    }
}
