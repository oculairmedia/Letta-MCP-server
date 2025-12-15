use alloc::string::String;
use core::str::FromStr;

use crate::{parsers, Date, Time};

/// Compound struct, holds Date and Time.
/// ```
/// # use std::str::FromStr;
/// assert_eq!(
///     iso8601::DateTime::from_str("2023-02-18T17:08:08.793Z"),
///     Ok(iso8601::DateTime {
///         date: iso8601::Date::YMD{ year: 2023, month: 2, day: 18},
///         time: iso8601::Time{ hour: 17, minute: 8, second: 8, millisecond: 793, tz_offset_hours: 0, tz_offset_minutes: 00 }
///     })
/// )
/// ```
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct DateTime {
    /// The date part
    pub date: Date,
    /// The time part
    pub time: Time,
}

impl FromStr for DateTime {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        datetime(s)
    }
}

/// Parses a datetime string.
///
/// A datetime string is a combination of the valid formats for the date and time,
/// separated by a literal `T`.
/// See the respective functions for the correct format.
///
/// ## Example
///
/// ```rust
/// let dt = iso8601::datetime("2015-11-03T21:56").unwrap();
/// ```
pub fn datetime(string: &str) -> Result<DateTime, String> {
    if let Ok((_left_overs, parsed)) = parsers::parse_datetime(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse datetime: {}", string))
    }
}
