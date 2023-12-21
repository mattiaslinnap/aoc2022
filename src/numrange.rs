use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct InclusiveRange {
    left: i32,
    right: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseRangeError;

impl InclusiveRange {
    pub fn new(left: i32, right: i32) -> Self {
        if left > right {
            panic!("InclusiveRange left must be <= right, got {left} > {right}");
        }

        Self { left, right }
    }

    /// Builds range from a string like "3-5".
    /// Cannot parse if second number is negative.
    pub fn parse(str_range: &str) -> Result<InclusiveRange, ParseRangeError> {
        Self::parse_with_separator(str_range, "-")
    }

    /// Builds range from a string like "3-5" or "-10..-3"
    pub fn parse_with_separator(str_range: &str, separator: &str) -> Result<InclusiveRange, ParseRangeError> {
        let pair = str_range.split_once(separator).ok_or(ParseRangeError)?;
        let left: i32 = pair.0.parse().map_err(|_| ParseRangeError)?;
        let right: i32 = pair.1.parse().map_err(|_| ParseRangeError)?;
        Ok(Self::new(left, right))
    }

    pub fn len(&self) -> i32 {
        self.right - self.left + 1
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.left <= other.left && other.right <= self.right
    }

    pub fn contains_value(&self, val: i32) -> bool {
        return self.left <= val && val <= self.right;
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        // Ranges overlap when either (or both) endpoint(s) from one is between the endpoints of another.
        return self.contains_value(other.left) || self.contains_value(other.right) ||
            other.contains_value(self.left) || other.contains_value(self.right);
    }
}

impl FromStr for InclusiveRange {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        InclusiveRange::parse(s)
    }
}

impl TryFrom<&str> for InclusiveRange {
    type Error = ParseRangeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        InclusiveRange::parse(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use crate::numrange::ParseRangeError;

    #[test]
    fn test_from_str() -> Result<(), ParseRangeError> {
        let result: InclusiveRange = "33-56".parse()?;
        assert_eq!(result, InclusiveRange::new(33, 56));
        return Ok(());
    }

    #[test]
    fn test_from_str_fail() {
        "33.-56".parse::<InclusiveRange>().expect_err("should not have parsed");
        "33-xx56".parse::<InclusiveRange>().expect_err("should not have parsed");
    }

    #[test]
    fn test_try_from_str() -> Result<(), ParseRangeError> {
        let result: InclusiveRange = InclusiveRange::try_from("33-56")?;
        assert_eq!(result, InclusiveRange::new(33, 56));
        return Ok(());
    }

    #[test]
    fn test_parse_with_separator() -> Result<(), ParseRangeError> {
        let result = InclusiveRange::parse_with_separator("-10..-3", "..")?;
        assert_eq!(result, InclusiveRange::new(-10, -3));
        return Ok(());
    }

    #[test]
    #[should_panic]
    fn test_left_right() {
        InclusiveRange::new(11, 10);
    }
}
