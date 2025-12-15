use core::fmt::{self, Display};

use super::{Date, DateTime, Duration, Time};

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // like `2015-11-02`
            Date::YMD { year, month, day } => write!(f, "{:04}-{:02}-{:02}", year, month, day),
            // like `2015-W45-01`
            Date::Week { year, ww, d } => write!(f, "{:04}-{:02}-{:02}", year, ww, d),
            // like `2015-306`
            Date::Ordinal { year, ddd } => write!(f, "{:04}-{:03}", year, ddd),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `16:43:16.123+00:00`
        write!(
            f,
            "{:02}:{:02}:{:02}.{}+{:02}:{:02}",
            self.hour,
            self.minute,
            self.second,
            self.millisecond,
            self.tz_offset_hours,
            self.tz_offset_minutes
        )
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `16:43:16.123+00:00`
        write!(f, "{}T{}", self.date, self.time)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Duration::YMDHMS {
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
            } => {
                if self.is_zero() {
                    write!(f, "P0D")?;
                    return Ok(());
                }

                write!(f, "P")?;

                if *year > 0 {
                    write!(f, "{}Y", year)?
                }

                if *month > 0 {
                    write!(f, "{}M", month)?
                }

                if *day > 0 {
                    write!(f, "{}D", day)?
                }

                if *hour > 0 || *minute > 0 || *second > 0 || *millisecond > 0 {
                    write!(f, "T")?
                }
                if *hour > 0 {
                    write!(f, "{}H", hour)?
                }
                if *minute > 0 {
                    write!(f, "{}M", minute)?
                }

                if *millisecond > 0 {
                    write!(f, "{}.{}S", second, millisecond)?
                } else if *second > 0 {
                    write!(f, "{}S", second)?
                }
                Ok(())
            }
            Duration::Weeks(w) => write!(f, "P{}W", w),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::parse_duration;

    use super::*;

    fn test_duration_reparse(duration: Duration) {
        let serialized = format!("{}", duration);
        let reparsed = parse_duration(serialized.as_bytes()).unwrap().1;
        assert_eq!(duration, reparsed);
    }

    #[test]
    fn display_duration_0() {
        let duration = Duration::YMDHMS {
            year: 2021,
            month: 11,
            day: 16,
            hour: 23,
            minute: 26,
            second: 59,
            millisecond: 0,
        };
        test_duration_reparse(duration);
    }

    #[test]
    fn display_duration_1() {
        let duration = Duration::YMDHMS {
            year: 2021,
            month: 11,
            day: 16,
            hour: 23,
            minute: 26,
            second: 59,
            millisecond: 123,
        };
        test_duration_reparse(duration);
    }

    #[test]
    fn display_duration_2() {
        let duration = Duration::Weeks(50);
        test_duration_reparse(duration);
    }
}
