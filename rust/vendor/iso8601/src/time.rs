use alloc::string::String;
use core::str::FromStr;

use crate::parsers;

/// A time object.
/// ```
/// # use std::str::FromStr;
/// assert_eq!(
///     iso8601::Time::from_str("17:08:08.793Z"),
///     Ok(iso8601::Time{ hour: 17, minute: 8, second: 8, millisecond: 793, tz_offset_hours: 0, tz_offset_minutes: 00 })
/// )
/// ```
#[derive(Eq, PartialEq, Debug, Copy, Clone, Default)]
pub struct Time {
    /// a 24th of a day
    pub hour: u32,
    /// 60 discrete parts of an hour
    pub minute: u32,
    /// a minute are 60 of these
    pub second: u32,
    /// everything after a `.`
    pub millisecond: u32,
    /// the hour part of the timezone offset from UTC
    pub tz_offset_hours: i32,
    /// the minute part of the timezone offset from UTC
    pub tz_offset_minutes: i32,
}

impl Time {
    /// Change this time's timezone offset.
    ///
    /// # Arguments
    ///
    /// * `tzo` - A tuple of `(hours, minutes)` specifying the timezone offset from UTC.
    pub fn set_tz(&self, tzo: (i32, i32)) -> Time {
        let mut t = *self;
        t.tz_offset_hours = tzo.0;
        t.tz_offset_minutes = tzo.1;
        t
    }
}

impl FromStr for Time {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        time(s)
    }
}

/// Parses a time string.
///
/// A string can have one of the following formats:
///
/// * `07:35:[00][.123]` or `0735[00][.123]`
/// * `07:35:[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)00:00)]`
/// * `0735[00][.123][(Z|(+|-)0000)]`
///
/// ## Example
///
/// ```rust
/// let time = iso8601::time("21:56:42").unwrap();
/// ```
pub fn time(string: &str) -> Result<Time, String> {
    if let Ok((_, parsed)) = parsers::parse_time(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse time: {}", string))
    }
}
