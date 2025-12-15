use super::*;
use crate::assert_parser;

#[test]
fn test_date_year() {
    assert_eq!(Ok((&[][..], 2015)), date_year(b"2015"));
    assert_eq!(Ok((&[][..], -333)), date_year(b"-0333"));
    assert_eq!(Ok((&b"-"[..], 2015)), date_year(b"2015-"));
    date_year(b"2023").unwrap();
    date_year(b"+2023").unwrap();
    date_year(b"-2023").unwrap();
    assert!(date_year(b"abcd").is_err());
    assert!(date_year(b"2a03").is_err());
}

#[test]
fn test_date_month() {
    assert_eq!(Ok((&[][..], 1)), date_month(b"01"));
    assert_eq!(Ok((&[][..], 6)), date_month(b"06"));
    assert_eq!(Ok((&[][..], 12)), date_month(b"12"));
    assert_eq!(Ok((&b"-"[..], 12)), date_month(b"12-"));

    assert!(date_month(b"13").is_err());
    assert!(date_month(b"00").is_err());
}

#[test]
fn test_date_day() {
    assert_eq!(Ok((&[][..], 1)), date_day(b"01"));
    assert_eq!(Ok((&[][..], 12)), date_day(b"12"));
    assert_eq!(Ok((&[][..], 20)), date_day(b"20"));
    assert_eq!(Ok((&[][..], 28)), date_day(b"28"));
    assert_eq!(Ok((&[][..], 30)), date_day(b"30"));
    assert_eq!(Ok((&[][..], 31)), date_day(b"31"));
    assert_eq!(Ok((&b"-"[..], 31)), date_day(b"31-"));

    assert!(date_day(b"00").is_err());
    assert!(date_day(b"32").is_err());
}

#[test]
fn test_time_hour() {
    assert_eq!(Ok((&[][..], 0)), time_hour(b"00"));
    assert_eq!(Ok((&[][..], 1)), time_hour(b"01"));
    assert_eq!(Ok((&[][..], 6)), time_hour(b"06"));
    assert_eq!(Ok((&[][..], 12)), time_hour(b"12"));
    assert_eq!(Ok((&[][..], 13)), time_hour(b"13"));
    assert_eq!(Ok((&[][..], 20)), time_hour(b"20"));
    assert_eq!(Ok((&[][..], 24)), time_hour(b"24"));

    assert!(time_hour(b"25").is_err());
    assert!(time_hour(b"30").is_err());
    assert!(time_hour(b"ab").is_err());
}

#[test]
fn test_time_minute() {
    assert_eq!(Ok((&[][..], 0)), time_minute(b"00"));
    assert_eq!(Ok((&[][..], 1)), time_minute(b"01"));
    assert_eq!(Ok((&[][..], 30)), time_minute(b"30"));
    assert_eq!(Ok((&[][..], 59)), time_minute(b"59"));

    assert!(time_minute(b"60").is_err());
    assert!(time_minute(b"61").is_err());
    assert!(time_minute(b"ab").is_err());
}

#[test]
fn test_time_second() {
    assert_eq!(Ok((&[][..], 0)), time_second(b"00"));
    assert_eq!(Ok((&[][..], 1)), time_second(b"01"));
    assert_eq!(Ok((&[][..], 30)), time_second(b"30"));
    assert_eq!(Ok((&[][..], 59)), time_second(b"59"));
    assert_eq!(Ok((&[][..], 60)), time_second(b"60"));

    assert!(time_second(b"61").is_err());
    assert!(time_second(b"ab").is_err());
}

#[test]
fn test_date() {
    assert!(parse_date(b"201").is_err());
    assert!(parse_date(b"2015p00p00").is_err());
    assert!(parse_date(b"pppp").is_err());
}

#[test]
fn test_time() {
    assert!(parse_time(b"20:").is_err());
    assert!(parse_time(b"20p42p16").is_err());
    assert!(parse_time(b"pppp").is_err());
}

#[test]
fn test_time_with_timezone() {
    assert!(parse_time(b"20:").is_err());
    assert!(parse_time(b"20p42p16").is_err());
    assert!(parse_time(b"pppp").is_err());
}

#[test]
fn test_date_iso_week_date() {
    assert!(date_iso_week(b"2015-W06-8").is_err());
    assert!(date_iso_week(b"2015-W068").is_err());
    assert!(date_iso_week(b"2015-W06-0").is_err());
    assert!(date_iso_week(b"2015-W00-2").is_err());
    assert!(date_iso_week(b"2015-W54-2").is_err());
    assert!(date_iso_week(b"2015-W542").is_err());
}

#[test]
fn test_date_ordinal_date() {
    // not valid here either
    assert!(date_ordinal(b"2015-400").is_err());
}

#[test]
fn format_equivalence() {
    assert_eq!(
        parse_datetime(b"2001-02-03T04:05:06+07:00"),
        parse_datetime(b"20010203T040506+0700")
    );
    assert_eq!(
        parse_datetime(b"2001-02-03T04:05:06+07:00"),
        parse_datetime(b"20010203T04:05:06+0700")
    );
    assert_eq!(
        parse_datetime(b"2001-02-03T04:05:00+07:00"),
        parse_datetime(b"20010203T0405+0700")
    );
    assert_eq!(
        parse_datetime(b"20010203T0405+0700"),
        parse_datetime(b"2001-02-03T0405+0700")
    );
    assert_eq!(
        parse_datetime(b"20010203T040506+0700"),
        parse_datetime(b"2001-02-03T040506+0700")
    );
    assert_eq!(
        parse_datetime(b"20010203T040506+0000"),
        parse_datetime(b"20010203T040506Z")
    );
    assert_eq!(
        parse_datetime(b"2015W056T04:05:06+07:00"),
        parse_datetime(b"2015-W05-6T04:05:06+07:00")
    );
}

#[test]
fn test_datetime_error() {
    let test_datetimes = vec!["ppp", "dumd-di-duTmd:iu:m"];

    for iso_string in test_datetimes {
        let res = parse_datetime(iso_string.as_bytes());
        assert!(res.is_err());
    }
}

#[test]
fn disallows_notallowed() {
    assert!(parse_time(b"30:90:90").is_err());
    assert!(parse_date(b"0000-20-40").is_err());
    assert!(parse_datetime(b"2001-w05-6t04:05:06.123z").is_err());
}

#[test]
fn test_duration_year() {
    assert_eq!(Ok((&[][..], 2019)), duration_year(b"2019Y"));
    assert_eq!(Ok((&[][..], 0)), duration_year(b"0Y"));
    assert_eq!(Ok((&[][..], 10000)), duration_year(b"10000Y"));
    assert!(duration_year(b"abcd").is_err());
    assert!(duration_year(b"-1").is_err());
}

#[test]
fn test_duration_month() {
    assert_eq!(Ok((&[][..], 6)), duration_month(b"6M"));
    assert_eq!(Ok((&[][..], 0)), duration_month(b"0M"));
    assert_eq!(Ok((&[][..], 12)), duration_month(b"12M"));
    assert!(duration_month(b"ab").is_err());
    assert!(duration_month(b"-1").is_err());
    assert!(duration_month(b"13").is_err());
}

#[test]
fn test_duration_week() {
    assert_eq!(Ok((&[][..], 26)), duration_week(b"26W"));
    assert_eq!(Ok((&[][..], 0)), duration_week(b"0W"));
    assert_eq!(Ok((&[][..], 52)), duration_week(b"52W"));
    assert!(duration_week(b"ab").is_err());
    assert!(duration_week(b"-1").is_err());
    assert!(duration_week(b"53").is_err());
}

#[test]
fn test_duration_day() {
    assert_eq!(Ok((&[][..], 16)), duration_day(b"16D"));
    assert_eq!(Ok((&[][..], 0)), duration_day(b"0D"));
    assert_eq!(Ok((&[][..], 31)), duration_day(b"31D"));
    assert!(duration_day(b"ab").is_err());
    assert!(duration_day(b"-1").is_err());
    assert!(duration_day(b"32").is_err());
}

#[test]
fn test_duration_hour() {
    assert_eq!(Ok((&[][..], 12)), duration_hour(b"12H"));
    assert_eq!(Ok((&[][..], 0)), duration_hour(b"0H"));
    assert_eq!(Ok((&[][..], 24)), duration_hour(b"24H"));
    assert!(duration_hour(b"ab").is_err());
    assert!(duration_hour(b"-1").is_err());
    assert!(duration_hour(b"25").is_err());
}

#[test]
fn test_duration_minute() {
    assert_eq!(Ok((&[][..], 30)), duration_minute(b"30M"));
    assert_eq!(Ok((&[][..], 0)), duration_minute(b"0M"));
    assert_eq!(Ok((&[][..], 60)), duration_minute(b"60M"));
    assert!(duration_minute(b"ab").is_err());
    assert!(duration_minute(b"-1").is_err());
    assert!(duration_minute(b"61").is_err());
}

#[test]
fn test_duration_second_and_millisecond1() {
    assert_eq!(
        Ok((&[][..], (30, 0))),
        duration_second_and_millisecond(b"30S")
    );
    assert_eq!(
        Ok((&[][..], (0, 0))),
        duration_second_and_millisecond(b"0S")
    );
    assert_eq!(
        Ok((&[][..], (60, 0))),
        duration_second_and_millisecond(b"60S")
    );
    assert_eq!(
        Ok((&[][..], (1, 230))),
        duration_second_and_millisecond(b"1,23S")
    );
    assert_eq!(
        Ok((&[][..], (2, 340))),
        duration_second_and_millisecond(b"2.34S")
    );
    assert!(duration_second_and_millisecond(b"abS").is_err());
    assert!(duration_second_and_millisecond(b"-1S").is_err());
}

#[test]
fn test_duration_time() {
    assert_eq!(Ok((&[][..], (1, 2, 3, 0))), duration_time(b"1H2M3S"));
    assert_eq!(Ok((&[][..], (10, 12, 30, 0))), duration_time(b"10H12M30S"));
    assert_eq!(Ok((&[][..], (1, 0, 3, 0))), duration_time(b"1H3S"));
    assert_eq!(Ok((&[][..], (0, 2, 0, 0))), duration_time(b"2M"));
    assert_eq!(Ok((&[][..], (1, 2, 3, 400))), duration_time(b"1H2M3,4S"));
    assert_eq!(Ok((&[][..], (1, 2, 3, 400))), duration_time(b"1H2M3.4S"));
    assert_eq!(Ok((&[][..], (0, 0, 0, 123))), duration_time(b"0,123S"));
    assert_eq!(Ok((&[][..], (0, 0, 0, 123))), duration_time(b"0.123S"));
}

#[test]
fn test_duration_ymdhms_error() {
    assert!(duration_ymdhms(b"").is_err());
    assert!(duration_ymdhms(b"P").is_err()); // empty duration is not 0 seconds
    assert!(duration_ymdhms(b"1Y2M3DT4H5M6S").is_err()); // missing P at start
    assert!(duration_ymdhms(b"T4H5M6S").is_err()); // missing P, required even if no YMD part
}

#[test]
fn test_duration_weeks_error() {
    assert!(duration_weeks(b"").is_err());
    assert!(duration_weeks(b"P").is_err()); // empty duration is not 0 seconds
    assert!(duration_weeks(b"P1").is_err()); // missing W after number
    assert!(duration_weeks(b"PW").is_err()); // missing number
}

#[test]
fn test_duration_datetime_error() {
    assert!(duration_datetime(b"").is_err());
    assert!(duration_datetime(b"P").is_err()); // empty duration is not 0 seconds
    assert!(duration_datetime(b"0001-02-03T04:05:06").is_err()); // missing P at start
}

#[rustfmt::skip]
#[test]
fn test_duration_second_and_millisecond2() {
    assert_parser!(
        parse_duration, "PT30S", 
        Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 30, millisecond: 0 }

    );

    assert_parser!(
        parse_duration, "PT30.123S", 
        Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 30, millisecond: 123 }

    );

    assert_parser!(
        parse_duration, "P2021Y11M16DT23H26M59.123S",
        Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 123 }
    );
}

#[rustfmt::skip]
#[test]
fn duration_roundtrip() {
    assert_parser!(
        parse_duration, "P0W", Duration::Weeks(0)
    );

    assert_parser!(
        parse_duration, "P2021Y11M16DT23H26M59.123S",
        Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 123 }
    );
    assert_parser!(
        parse_duration, "P2021Y11M16DT23H26M59S",
        Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 59, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "P2021Y11M16DT23H26M",
        Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 26, second: 0, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "P2021Y11M16DT23H",
        Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 23, minute: 0, second: 0, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "P2021Y11M16D",
        Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 0, minute: 0, second: 0, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "P2021Y11M16DT1S",
        Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 0, minute: 0, second: 1, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "P2021Y11M16DT0.471S",
        Duration::YMDHMS { year: 2021, month: 11, day: 16, hour: 0, minute: 0, second: 0, millisecond: 471 }
    );
    assert_parser!(
        parse_duration, "P2021Y11M",
        Duration::YMDHMS { year: 2021, month: 11, day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "P11M",
        Duration::YMDHMS { year: 0, month: 11, day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "P16D",
        Duration::YMDHMS { year: 0, month: 0, day: 16, hour: 0, minute: 0, second: 0, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "P0D",
        Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 0, millisecond: 0 }
    );
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_hour() {
    assert_parser!(
        parse_duration, "PT12H",
        Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 12, minute: 0, second: 0, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "PT8760H",
        Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 365*24, minute: 0, second: 0, millisecond: 0 }
    );
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_minute() {
    assert_parser!(
        parse_duration, "PT15M",
        Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 15, second: 0, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "PT600M",
        Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 600, second: 0, millisecond: 0 }
    );
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_second() {
    assert_parser!(
        parse_duration, "PT16S",
        Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 16, millisecond: 0 }
    );

    assert_parser!(
        parse_duration, "PT900S",
        Duration::YMDHMS { year: 0, month: 0, day: 0, hour: 0, minute: 0, second: 900, millisecond: 0 }
    );
}

#[rustfmt::skip]
#[test]
fn duration_multi_digit_day() {
    assert_parser!(
        parse_duration, "P365D",
        Duration::YMDHMS { year: 0, month: 0, day: 365, hour: 0, minute: 0, second: 0, millisecond: 0 }
    );
    assert_parser!(
        parse_duration, "P36500D",
        Duration::YMDHMS { year: 0, month: 0, day: 36500, hour: 0, minute: 0, second: 0, millisecond: 0 }
    );
}

// #[test]
// fn corner_cases() {
//    // how to deal with left overs?
//    assert!(parse_datetime((b"2015-06-26T22:57:09Z00:00").is_done());
//    assert!(date("2015-06-26T22:57:09Z00:00").is_err());
//
//    assert!(parse_datetime((b"2015-06-26T22:57:09Z+00:00").is_done());
//    assert!(datetime("2015-06-26T22:57:09Z+00:00").is_err());
//    assert!(parse_datetime((b"2001-W05-6T04:05:06.123455Z").is_err());
//    assert!(parse_datetime((b"2015-06-26TZ").is_err());
// }

#[test]
#[ignore]
/// a few things we probably don't parse correctly yet
/// see <https://ijmacd.github.io/rfc3339-iso8601/>
fn iso8601_vs_rfc3339() {
    // "+002023-02-18".parse::<Date>().unwrap();  // six digit years
    // "+002023-02".parse::<Date>().unwrap(); // six digit years
    // "+002023".parse::<Date>().unwrap(); // six digit years
    // "+2023".parse::<Date>().unwrap(); // missing months etc
    // "2023-02-18 18:29:24+01:00".parse::<DateTime>().unwrap();
    // "2023-02-18_17:29:49.278Z".parse::<DateTime>().unwrap();
    // "2021-208T22:20:32.332320+08".parse::<DateTime>().unwrap();
}
