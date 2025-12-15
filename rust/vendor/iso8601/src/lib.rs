//! ISO8601 is a parser library for the
//! [ISO8601](https://en.wikipedia.org/wiki/ISO_8601) format
//! and partially RFC3339.
//!
//! Validity of a given date is not guaranteed, this parser will happily parse
//! `"2015-02-29"` as a valid date,
//! even though 2015 was no leap year.
//!
//! # Example
//!
//! ```rust
//! let datetime = iso8601::datetime("2015-06-26T16:43:23+0200").unwrap();
//! let time = "16:43:23+0200".parse::<iso8601::Time>().unwrap();
//! let date = "2015-02-29".parse::<iso8601::Date>().unwrap();
//! let datetime = "2015-06-26T16:43:23+0200".parse::<iso8601::DateTime>().unwrap();
//! let duration = "P2021Y11M16DT23H26M59.123S".parse::<iso8601::Duration>().unwrap();
//! ```

#![allow(clippy::uninlined_format_args)]
#![deny(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    missing_docs
)]
#![warn(clippy::doc_markdown)]
#![no_std]

#[cfg(any(feature = "std", test))]
#[macro_use]
extern crate std;

#[macro_use]
extern crate alloc;

mod display;
pub mod parsers;

mod date;
pub use date::{date, Date};

mod time;
pub use time::{time, Time};

mod datetime;
pub use datetime::{datetime, DateTime};

mod duration;
pub use duration::{duration, Duration};

#[cfg(feature = "chrono")]
mod chrono;

#[cfg(feature = "serde")]
mod serde;

#[cfg(test)]
mod assert;
