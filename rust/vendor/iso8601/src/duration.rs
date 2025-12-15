use core::str::FromStr;

use alloc::string::String;

use crate::parsers;

/// A time duration.
/// Durations:
/// <https://www.rfc-editor.org/rfc/rfc3339#page-13>
///    dur-second        = 1*DIGIT "S"
///    dur-minute        = 1*DIGIT "M" [dur-second]
///    dur-hour          = 1*DIGIT "H" [dur-minute]
///    dur-time          = "T" (dur-hour / dur-minute / dur-second)
///    dur-day           = 1*DIGIT "D"
///    dur-week          = 1*DIGIT "W"
///    dur-month         = 1*DIGIT "M" [dur-day]
///    dur-year          = 1*DIGIT "Y" [dur-month]
///    dur-date          = (dur-day / dur-month / dur-year) [dur-time]
///    duration          = "P" (dur-date / dur-time / dur-week)
/// ```
///# use std::str::FromStr;
/// assert_eq!(iso8601::Duration::from_str("P2021Y11M16DT23H26M59.123S"), Ok(iso8601::Duration::YMDHMS{ year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 123 }))
/// ```
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Duration {
    /// A duration specified by year, month, day, hour, minute and second units
    YMDHMS {
        /// Number of calendar years
        year: u32,
        /// Number of months
        month: u32,
        /// Number of days
        day: u32,
        /// Number of hours
        hour: u32,
        /// Number of minutes
        minute: u32,
        /// Number of seconds
        second: u32,
        /// Number of milliseconds
        millisecond: u32,
    },
    /// consists of week units
    Weeks(u32),
}

impl Duration {
    /// Whether this duration represents a zero duration.
    pub fn is_zero(&self) -> bool {
        *self
            == Duration::YMDHMS {
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                minute: 0,
                second: 0,
                millisecond: 0,
            }
            || *self == Duration::Weeks(0)
    }
}

impl Default for Duration {
    fn default() -> Duration {
        Duration::YMDHMS {
            year: 0,
            month: 0,
            day: 0,
            hour: 0,
            minute: 0,
            second: 0,
            millisecond: 0,
        }
    }
}

impl FromStr for Duration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        duration(s)
    }
}

impl From<Duration> for ::core::time::Duration {
    fn from(duration: Duration) -> Self {
        match duration {
            Duration::YMDHMS {
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
            } => {
                let secs = u64::from(year) * 365 * 86_400
                    + u64::from(month) * 30 * 86_400
                    + u64::from(day) * 86_400
                    + u64::from(hour) * 3600
                    + u64::from(minute) * 60
                    + u64::from(second);
                let nanos = millisecond * 1_000_000;
                Self::new(secs, nanos)
            }
            Duration::Weeks(week) => {
                let secs = u64::from(week) * 7 * 86_400;
                Self::from_secs(secs)
            }
        }
    }
}

/// Parses a duration string.
///
/// A string starts with `P` and can have one of the following formats:
///
/// * Fully-specified duration: `P1Y2M3DT4H5M6S`
/// * Duration in weekly intervals: `P1W`
/// * Fully-specified duration in [`DateTime`](`crate::DateTime`) format: `P<datetime>`
///
/// Both fully-specified formats get parsed into the YMDHMS Duration variant.
/// The weekly interval format gets parsed into the Weeks Duration variant.
///
/// The ranges for each of the individual units are not expected to exceed
/// the next largest unit.
///
/// These ranges (inclusive) are as follows:
///
/// * Year (any valid u32)
/// * Month 0 - 12
/// * Week 0 - 52
/// * Day 0 - 31
/// * Hour 0 - 24
/// * Minute 0 - 60
/// * Second 0 - 60
///
/// ## Examples
///
/// ```rust
/// let duration = iso8601::duration("P1Y2M3DT4H5M6S").unwrap();
/// let duration = iso8601::duration("P1W").unwrap();
/// let duration = iso8601::duration("P2015-11-03T21:56").unwrap();
/// ```
pub fn duration(string: &str) -> Result<Duration, String> {
    if let Ok((_left_overs, parsed)) = parsers::parse_duration(string.as_bytes()) {
        Ok(parsed)
    } else {
        Err(format!("Failed to parse duration: {}", string))
    }
}
