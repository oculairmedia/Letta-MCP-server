use alloc::string::String;
use core::str::FromStr;

use crate::parsers;

/// A date, can hold three different formats.
/// ```
/// # use std::str::FromStr;
/// assert_eq!(
///     iso8601::Date::from_str("2023-02-18T17:08:08.793Z"),
///     Ok(iso8601::Date::YMD{ year: 2023, month: 2, day: 18})
/// )
/// ```
#[allow(missing_docs)]
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Date {
    /// consists of year, month and day of month
    YMD { year: i32, month: u32, day: u32 },
    /// consists of year, week and day of week
    Week { year: i32, ww: u32, d: u32 },
    /// consists of year and day of year
    Ordinal { year: i32, ddd: u32 },
}

impl Default for Date {
    fn default() -> Date {
        Date::YMD {
            year: 0,
            month: 0,
            day: 0,
        }
    }
}

impl FromStr for Date {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        date(s)
    }
}

/// Parses a date string.
///
/// A string can have one of the following formats:
///
/// * `2015-11-02` or `20151102`
/// * `2015-W45-01` or `2015W451`
/// * `2015-306` or `2015306`
///
/// ## Example
///
/// ```rust
/// let date = iso8601::date("2015-11-02").unwrap();
/// ```
pub fn date(string: &str) -> Result<Date, String> {
    if let Ok((_, parsed)) = parsers::parse_date(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse date: {}", string))
    }
}
