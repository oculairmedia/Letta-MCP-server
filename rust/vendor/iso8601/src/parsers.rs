//! The low-level parsers for date, datetime, duration and time.
//!
//! The top-level functions [`date()`][`crate::date()`], [`datetime()`][`crate::datetime()`],
//! [`duration()`][`crate::duration()`] and [`time()`][`crate::time()`]
//! provide convenient wrappers around the low-level parsers,
//! but throw away leftover input on success.
//!
//! Using the low-level functions provided here allows to recover leftover input
//! or to combine these parsers with other parser combinators.

use core::str;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::one_of,
    combinator::{map_res, not, opt},
    error::Error,
    sequence::{preceded, separated_pair, terminated},
    AsChar, Err, IResult, Parser,
};

use crate::{Date, DateTime, Duration, Time};

#[cfg(test)]
mod tests;

// UTILITY

fn take_digits(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, digits) = take_while(AsChar::is_dec_digit).parse(i)?;

    if digits.is_empty() {
        return Err(Err::Error(Error::new(i, nom::error::ErrorKind::Eof)));
    }

    let s = str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
    let res = s
        .parse()
        .expect("Invalid string, expected ASCII representation of a number");

    Ok((i, res))
}

fn take_n_digits(i: &[u8], n: usize) -> IResult<&[u8], u32> {
    let (i, digits) = take_while_m_n(n, n, AsChar::is_dec_digit)(i)?;

    let s = str::from_utf8(digits).expect("Invalid data, expected UTF-8 string");
    let res = s
        .parse()
        .expect("Invalid string, expected ASCII representation of a number");

    Ok((i, res))
}

fn n_digit_in_range(
    i: &[u8],
    n: usize,
    range: impl core::ops::RangeBounds<u32>,
) -> IResult<&[u8], u32> {
    let (new_i, number) = take_n_digits(i, n)?;

    if range.contains(&number) {
        Ok((new_i, number))
    } else {
        Err(Err::Error(Error::new(i, nom::error::ErrorKind::Eof)))
    }
}

fn sign(i: &[u8]) -> IResult<&[u8], i32> {
    alt((tag("-"), tag("+")))
        .map(|s: &[u8]| match s {
            b"-" => -1,
            _ => 1,
        })
        .parse(i)
}

// DATE

// [+/-]YYYY
fn date_year(i: &[u8]) -> IResult<&[u8], i32> {
    // The sign is optional, but defaults to `+`
    (
        opt(sign),               // [+/-]
        |i| take_n_digits(i, 4), // year
    )
        .map(|(s, year)| s.unwrap_or(1) * year as i32)
        .parse(i)
}

// MM
fn date_month(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 1..=12)
}

// DD
fn date_day(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 1..=31)
}

// WW
fn date_week(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 1..=52)
}

fn date_week_day(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 1, 1..=7)
}

// ordinal DDD
fn date_ord_day(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 3, 1..=366)
}

// YYYY-MM-DD
fn date_ymd(i: &[u8]) -> IResult<&[u8], Date> {
    (
        date_year,     // YYYY
        opt(tag("-")), // -
        date_month,    // MM
        opt(tag("-")), // -
        date_day,      //DD
    )
        .map(|(year, _, month, _, day)| Date::YMD { year, month, day })
        .parse(i)
}

// YYYY-DDD
fn date_ordinal(i: &[u8]) -> IResult<&[u8], Date> {
    separated_pair(date_year, opt(tag("-")), date_ord_day)
        .map(|(year, ddd)| Date::Ordinal { year, ddd })
        .parse(i)
}

// YYYY-"W"WW-D
fn date_iso_week(i: &[u8]) -> IResult<&[u8], Date> {
    (
        date_year,                 // y
        (opt(tag("-")), tag("W")), // [-]W
        date_week,                 // w
        opt(tag("-")),             // [-]
        date_week_day,             // d
    )
        .map(|(year, _, ww, _, d)| Date::Week { year, ww, d })
        .parse(i)
}

/// Parses a date string.
///
/// See [`date()`][`crate::date()`] for the supported formats.
pub fn parse_date(i: &[u8]) -> IResult<&[u8], Date> {
    alt((date_ymd, date_iso_week, date_ordinal)).parse(i)
}

// TIME

// HH
fn time_hour(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 0..=24)
}

// MM
fn time_minute(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 0..=59)
}

// SS
fn time_second(i: &[u8]) -> IResult<&[u8], u32> {
    n_digit_in_range(i, 2, 0..=60)
}

// Converts the fractional part if-any of a number of seconds to milliseconds
// truncating towards zero if there are more than three digits.
// e.g. "" -> 0, "1" -> 100, "12" -> 120, "123" -> 123, "1234" -> 123
fn fraction_millisecond(i: &[u8]) -> IResult<&[u8], u32> {
    let (i, mut digits) = take_while(AsChar::is_dec_digit).parse(i)?;
    let mut l = digits.len();
    if l > 3 {
        digits = digits.get(0..3).unwrap();
    }
    let mut result = 0;
    if l > 0 {
        let digits = str::from_utf8(digits).unwrap(); // This can't panic, `digits` will only include digits.
        result = digits.parse().unwrap();
    }
    while l < 3 {
        result *= 10;
        l += 1;
    }
    Ok((i, result))
}

/// Parses a time string.
///
/// See [`time()`][`crate::time()`] for the supported formats.
// HH:MM:[SS][.(m*)][(Z|+...|-...)]
pub fn parse_time(i: &[u8]) -> IResult<&[u8], Time> {
    (
        time_hour,                                         // HH
        opt(tag(":")),                                     // :
        time_minute,                                       // MM
        opt(preceded(opt(tag(":")), time_second)),         // [SS]
        opt(preceded(one_of(",."), fraction_millisecond)), // [.(m*)]
        opt(alt((timezone_hour, timezone_utc))),           // [(Z|+...|-...)]
    )
        .map(|(h, _, m, s, ms, z)| {
            let (tz_offset_hours, tz_offset_minutes) = z.unwrap_or((0, 0));

            Time {
                hour: h,
                minute: m,
                second: s.unwrap_or(0),
                millisecond: ms.unwrap_or(0),
                tz_offset_hours,
                tz_offset_minutes,
            }
        })
        .parse(i)
}

fn timezone_hour(i: &[u8]) -> IResult<&[u8], (i32, i32)> {
    (sign, time_hour, opt(preceded(opt(tag(":")), time_minute)))
        .map(|(s, h, m)| (s * (h as i32), s * (m.unwrap_or(0) as i32)))
        .parse(i)
}

fn timezone_utc(input: &[u8]) -> IResult<&[u8], (i32, i32)> {
    tag("Z").map(|_| (0, 0)).parse(input)
}

/// Parses a datetime string.
///
/// See [`datetime()`][`crate::datetime()`] for supported formats.
// Full ISO8601 datetime
pub fn parse_datetime(i: &[u8]) -> IResult<&[u8], DateTime> {
    separated_pair(parse_date, tag("T"), parse_time)
        .map(|(d, t)| DateTime { date: d, time: t })
        .parse(i)
}

// DURATION

///    dur-year          = 1*DIGIT "Y" [dur-month]
fn duration_year(i: &[u8]) -> IResult<&[u8], u32> {
    terminated(take_digits, tag("Y")).parse(i)
}

///    dur-month         = 1*DIGIT "M" [dur-day]
fn duration_month(i: &[u8]) -> IResult<&[u8], u32> {
    terminated(take_digits, tag("M")).parse(i)
}

///    dur-week          = 1*DIGIT "W"
fn duration_week(i: &[u8]) -> IResult<&[u8], u32> {
    terminated(take_digits, tag("W")).parse(i)
}

//    dur-day           = 1*DIGIT "D"
fn duration_day(i: &[u8]) -> IResult<&[u8], u32> {
    terminated(take_digits, tag("D")).parse(i)
}

///    dur-hour          = 1*DIGIT "H" [dur-minute]
///    dur-time          = "T" (dur-hour / dur-minute / dur-second)
fn duration_hour(i: &[u8]) -> IResult<&[u8], u32> {
    terminated(take_digits, tag("H")).parse(i)
}

///    dur-minute        = 1*DIGIT "M" [dur-second]
fn duration_minute(i: &[u8]) -> IResult<&[u8], u32> {
    terminated(take_digits, tag("M")).parse(i)
}

///    dur-second        = 1*DIGIT "S"
fn duration_second(i: &[u8]) -> IResult<&[u8], u32> {
    terminated(take_digits, tag("S")).parse(i)
}

///    dur-second-ext    = 1*DIGIT (,|.) 1*DIGIT "S"
fn duration_second_and_millisecond(i: &[u8]) -> IResult<&[u8], (u32, u32)> {
    alt((
        // no milliseconds
        duration_second.map(|m| (m, 0)),
        terminated(
            // with milliseconds
            separated_pair(take_digits, one_of(",."), fraction_millisecond),
            tag("S"),
        ),
    ))
    .parse(i)
}

fn duration_time(i: &[u8]) -> IResult<&[u8], (u32, u32, u32, u32)> {
    (
        opt(duration_hour),
        opt(duration_minute),
        opt(duration_second_and_millisecond),
    )
        .map(|(h, m, s)| {
            let (s, ms) = s.unwrap_or((0, 0));

            (h.unwrap_or(0), m.unwrap_or(0), s, ms)
        })
        .parse(i)
}

fn duration_ymdhms(i: &[u8]) -> IResult<&[u8], Duration> {
    map_res(
        preceded(
            tag("P"),
            (
                opt(duration_year),
                opt(duration_month),
                opt(duration_day),
                opt(preceded(tag("T"), duration_time)),
            ),
        ),
        |(y, mo, d, time)| {
            // at least one element must be present for a valid duration representation
            if y.is_none() && mo.is_none() && d.is_none() && time.is_none() {
                return Err((i, nom::error::ErrorKind::Eof));
            }

            let (h, mi, s, ms) = time.unwrap_or((0, 0, 0, 0));

            Ok(Duration::YMDHMS {
                year: y.unwrap_or(0),
                month: mo.unwrap_or(0),
                day: d.unwrap_or(0),
                hour: h,
                minute: mi,
                second: s,
                millisecond: ms,
            })
        },
    )
    .parse(i)
}

fn duration_weeks(i: &[u8]) -> IResult<&[u8], Duration> {
    preceded(tag("P"), duration_week)
        .map(Duration::Weeks)
        .parse(i)
}

// YYYY, no sign
fn duration_datetime_year(i: &[u8]) -> IResult<&[u8], u32> {
    take_n_digits(i, 4)
}

fn duration_datetime(i: &[u8]) -> IResult<&[u8], Duration> {
    preceded(
        (tag("P"), not(sign)),
        (
            duration_datetime_year,
            opt(tag("-")),
            date_month,
            opt(tag("-")),
            date_day,
            tag("T"),
            parse_time,
        ),
    )
    .map(|(year, _, month, _, day, _, t)| Duration::YMDHMS {
        year,
        month,
        day,
        hour: t.hour,
        minute: t.minute,
        second: t.second,
        millisecond: t.millisecond,
    })
    .parse(i)
}

/// Parses a duration string.
///
/// See [`duration()`][`crate::duration()`] for supported formats.
pub fn parse_duration(i: &[u8]) -> IResult<&[u8], Duration> {
    alt((duration_ymdhms, duration_weeks, duration_datetime)).parse(i)
}
