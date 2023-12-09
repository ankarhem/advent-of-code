use std::{error::Error, fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Year(u16);

impl Year {
    pub fn new(year: u16) -> Option<Self> {
        if !(2015..=2023).contains(&year) {
            return None;
        }
        Some(Self(year))
    }

    // Not part of the public API
    #[doc(hidden)]
    pub const fn __new_unchecked(year: u16) -> Self {
        Self(year)
    }

    /// Converts the [`Year`] into an [`u16`].
    pub fn into_inner(self) -> u16 {
        self.0
    }
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}", self.0)
    }
}

impl PartialEq<u16> for Year {
    fn eq(&self, other: &u16) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<u16> for Year {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

/* -------------------------------------------------------------------------- */

impl FromStr for Year {
    type Err = YearFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year = s.parse().map_err(|_| YearFromStrError)?;
        Self::new(year).ok_or(YearFromStrError)
    }
}

/// An error which can be returned when parsing a [`Year`].
#[derive(Debug)]
pub struct YearFromStrError;

impl Error for YearFromStrError {}

impl Display for YearFromStrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("expecting a year number between 2015 and 2024")
    }
}

/* -------------------------------------------------------------------------- */

/// An iterator that yields every aoc year.
pub fn all_years() -> AllYears {
    AllYears::new()
}

/// An iterator that yields every aoc year.
pub struct AllYears {
    current: u16,
}

impl AllYears {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { current: 1 }
    }
}

impl Iterator for AllYears {
    type Item = Year;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 2023 {
            return None;
        }
        // NOTE: the iterator starts at 1 and we have verified that the value is not above 25.
        let year = Year(self.current);
        self.current += 1;

        Some(year)
    }
}

/* -------------------------------------------------------------------------- */

/// Creates a [`Year`] value in a const context.
#[macro_export]
macro_rules! year {
    ($year:expr) => {{
        const _ASSERT: () = assert!(
            $year >= 2015 && $year <= 2023,
            concat!(
                "invalid year number `",
                $year,
                "`, expecting a value between 2015 and 2024"
            ),
        );
        $crate::Year::__new_unchecked($year)
    }};
}

/* -------------------------------------------------------------------------- */

#[cfg(feature = "test_lib")]
mod tests {
    use super::{all_years, Year};

    #[test]
    fn all_years_iterator() {
        let mut iter = all_years();

        assert_eq!(iter.next(), Some(Year(2015)));
        assert_eq!(iter.next(), Some(Year(2016)));
        assert_eq!(iter.next(), Some(Year(2017)));
        assert_eq!(iter.next(), Some(Year(2018)));
        assert_eq!(iter.next(), Some(Year(2019)));
        assert_eq!(iter.next(), Some(Year(2020)));
        assert_eq!(iter.next(), Some(Year(2021)));
        assert_eq!(iter.next(), Some(Year(2022)));
        assert_eq!(iter.next(), Some(Year(2023)));
        assert_eq!(iter.next(), None);
    }
}

/* -------------------------------------------------------------------------- */
