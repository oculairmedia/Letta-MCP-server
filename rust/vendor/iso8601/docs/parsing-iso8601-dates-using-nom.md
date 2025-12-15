# omnomnom - Parsing ISO8601 dates using nom

--

Authors:

- Jan-Erik Rediger
- Chris Couzens

History:

- 2015-07-16: [Original blog article](https://fnordig.de/2015/07/16/omnomnom-parsing-iso8601-dates-using-nom/)
- 2018-07-01: Imported into crate repository & updated article to work with nom 4.

--

There are thousands of ways to note down a date and time.
The international date format is standardized as [ISO8601][iso], though it still allows a widespread of different formats.

The basic format looks like this:

> 2015-07-02T19:45:00+0100

And that's what we will parse today using [nom][nom],
a parser combinator library created by [Geoffroy Couprie][gcouprie].

The idea is that you write small self-contained parsers, which all do only one simple thing, like parsing the year in our string, and then combine these small parsers to a bigger one to parse the full format.
`nom` comes with a wide variety of small parsers: handling different integers, reading simple byte arrays, optional fields, mapping parsed data over a function, ...
Most of them are provided as combinable macros.
It's very easy to implement your own small parsers, either by providing a method that handles a short byte buffer or by combining existing parsers.

So let's dive right in and see how to use nom in real code.

### Analysis

This is what we want to parse:

> 2015-07-02T19:45:00+0100

It has several parts we need to parse:

> YYYY-MM-DDTHH:MM:SS+OOOO

with the following meaning:

| Characters | Meaning                                                            |
| ---------- | ------------------------------------------------------------------ |
| YYYY       | The year, can be negative or null and can be extended if necessary |
| MM         | Month from 1 to 12 (0-prefixed)                                    |
| DD         | Day from 1 to 31 (0-prefixed)                                      |
| T          | Separator between date and time                                    |
| HH         | Hour, 0-23 (0-prefixed)                                            |
| MM         | Minutes, 0-59 (0-prefixed)                                         |
| SS         | Seconds, 0-59 (0-prefixed)                                         |
| OOOO       | Timezone offset, separated by a `+` or `-` sign or `Z` for UTC     |

Parts like the seconds and the timezone offset are optional. Datetime strings without them will default to a zero value for that field.
The date parts are separated by a dash (`-`) and the time parts by a colon (`:`).

We will built a small parser for each of these parts and at the end combine them to parse a full date time string.

### Boiler Plate

We will need to make a lib project.

```bash
cargo new --lib date_parse
```

Edit `Cargo.toml` and `src/lib.rs` so that our project depends on nom.

```toml
[dependencies]
nom = "^4.0"
```

```rust
#[macro_use]
extern crate nom;
```

### Parsing the date: 2015-07-16

Let's start with the sign. As we need it several times, we create its own parser for that.
Parsers are created by giving them a name, stating the return value (or defaulting to a byte slice) and the parser combinators to handle the input.

```rust
named!(sign <&[u8], i32>, alt!(
        tag!("-") => { |_| -1 } |
        tag!("+") => { |_| 1 }
        )
    );

#[cfg(test)]
mod tests {
    use nom::Context::Code;
    use nom::Err::Error;
    use nom::Err::Incomplete;
    use nom::ErrorKind::Alt;
    use nom::Needed::Size;
    use sign;

    #[test]
    fn parse_sign() {
        assert_eq!(sign(b"-"), Ok((&[][..], -1)));
        assert_eq!(sign(b"+"), Ok((&[][..], 1)));
        assert_eq!(sign(b""), Err(Incomplete(Size(1))));
        assert_eq!(sign(b" "), Err(Error(Code(&b" "[..], Alt))));
    }
}
```

First, we parse either a plus or a minus sign.
This combines two already existing parsers: `tag!`, which will match the given byte array (in our case a single character) and `alt!`, which will try a list of parsers, returning on the first successful one.
We can directly map the result of the sub-parsers to either `-1` or `1`, so we don't need to deal with the byte slice later.

Next we parse the year, which consists of an optional sign and 4 digits (I know, I know, it is possible to extend this to more digits, but let's keep it simple for now).

```rust
use std::ops::{AddAssign, MulAssign};

fn buf_to_int<T>(s: &[u8]) -> T
where
    T: AddAssign + MulAssign + From<u8>,
{
    let mut sum = T::from(0);
    for digit in s {
        sum *= T::from(10);
        sum += T::from(*digit - b'0');
    }
    sum
}

named!(positive_year  <&[u8], i32>, map!(take_while_m_n!(4, 4, nom::is_digit), buf_to_int));
named!(pub year <&[u8], i32>, do_parse!(
    pref: opt!(sign) >>
    y: positive_year >>
    (pref.unwrap_or(1) * y)
));

#[cfg(test)]
mod tests {
    use positive_year;
    use year;

    #[test]
    fn parse_positive_year() {
        assert_eq!(positive_year(b"2018"), Ok((&[][..], 2018)));
    }

    #[test]
    fn parse_year() {
        assert_eq!(year(b"2018"), Ok((&[][..], 2018)));
        assert_eq!(year(b"+2018"), Ok((&[][..], 2018)));
        assert_eq!(year(b"-2018"), Ok((&[][..], -2018)));
    }
}

```

A lot of additional stuff here. So let's separate it.

```rust
named!(positive_year  <&[u8], i32>, map!(take_while_m_n!(4, 4, nom::is_digit), buf_to_int));
```

This creates a new named parser, that again returns the remaining input and an 32-bit integer.
To work, it first calls `take_4_digits` and then maps that result to the corresponding integer.

`take_while_m_n` is another small helper parser. We will also use one for 2 digits:

```rust
take_while_m_n!(4, 4, nom::is_digit)
take_while_m_n!(2, 2, nom::is_digit)
```

This takes 4 (or 2) characters from the input and checks that each character is a digit.

```rust
named!(pub year <&[u8], i32>, do_parse!(
```

The year is also returned as a 32-bit integer (there's a pattern!).
Using the `do_parse!` macro, we can chain together multiple parsers and work with the sub-results.

```rust
    pref: opt!(sign) >>
    y: positive_year >>
```

Our sign is directly followed by 4 digits. It's optional though, that's why we use `opt!`.
`>>` is the concatenation operator in the `chain!` macro.
We save the sub-results to variables (`pref` and `y`).

```rust
    (pref.unwrap_or(1) * y)
```

To get the final result, we multiply the prefix (which comes back as either `1` or `-1`) with the year.

We can now successfully parse a year:

```rust
        assert_eq!(year(b"2018"), Ok((&[][..], 2018)));
        assert_eq!(year(b"-0333"), Ok((&[][..], -0333)));
```

Our nom parser will return an `IResult`.

```rust
type IResult<I, O, E = u32> = Result<(I, O), Err<I, E>>;
pub enum Err<I, E = u32> {
    Incomplete(Needed),
    Error(Context<I, E>),
    Failure(Context<I, E>),
}
```

If all went well, we get `Ok(I,O)` with `I` and `O` being the appropriate types.
For our case `I` is the same as the input, a buffer slice (`&[u8]`), and `O` is the output of the parser itself, an integer (`i32`).
The return value could also be an `Err(Failure)`, if something went completely wrong, or `Err(Incomplete(Needed))`, requesting more data to be able to satisfy the parser (you can't parse a 4-digit year with only 3 characters input).

Parsing the month and day is a bit easier now: we simply take the digits and map them to an integer:

```rust
named!(month <&[u8], u8>, map!(take_while_m_n!(2, 2, nom::is_digit), buf_to_int));
named!(day   <&[u8], u8>, map!(take_while_m_n!(2, 2, nom::is_digit), buf_to_int));

#[cfg(test)]
mod tests {
    use day;
    use month;

    #[test]
    fn parse_month() {
        assert_eq!(month(b"06"), Ok((&[][..], 06)));
    }

    #[test]
    fn parse_day() {
        assert_eq!(day(b"18"), Ok((&[][..], 18)));
    }
}
```

All that's left is combining these 3 parts to parse a full date.
Again we can chain the different parsers and map it to some useful value:

```rust
#[derive(Eq, PartialEq, Debug)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

named!(pub date <&[u8], Date>, do_parse!(
    year: year >>
    tag!("-") >>
    month: month >>
    tag!("-") >>
    day: day >>
    (Date { year, month, day})
));

#[cfg(test)]
mod tests {
    use date;
    use Date;

    #[test]
    fn parse_date() {
        assert_eq!(
            Ok((
                &[][..],
                Date {
                    year: 2015,
                    month: 7,
                    day: 16
                }
            )),
            date(b"2015-07-16")
        );
        assert_eq!(
            Ok((
                &[][..],
                Date {
                    year: -333,
                    month: 6,
                    day: 11
                }
            )),
            date(b"-0333-06-11")
        );
    }
}

```

And running the tests shows it already works!

### Parsing the time: 16:43:52

Next, we parse the time. The individual parts are really simple, just some digits:

```rust
named!(pub hour   <&[u8], u8>, map!(take_while_m_n!(2, 2, nom::is_digit), buf_to_int));
named!(pub minute <&[u8], u8>, map!(take_while_m_n!(2, 2, nom::is_digit), buf_to_int));
named!(pub second <&[u8], u8>, map!(take_while_m_n!(2, 2, nom::is_digit), buf_to_int));
```

Putting them together becomes a bit more complex, as the `second` part is optional:

```rust
#[derive(Eq, PartialEq, Debug)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: u8,
    tz_offset: i32,
}

named!(pub time <&[u8], Time>, do_parse!(
    hour: hour >>
    tag!(":") >>
    minute: minute >>
    second: opt!(complete!(do_parse!(
        tag!(":") >>
        second: second >>
        (second)
    ))) >>
    (Time {hour, minute, second: second.unwrap_or(0), tz_offset: 0})
));

#[cfg(test)]
mod tests {
    use time;
    use Time;

    #[test]
    fn parse_time() {
        assert_eq!(
            Ok((
                &[][..],
                Time {
                    hour: 16,
                    minute: 43,
                    second: 52,
                    tz_offset: 0
                }
            )),
            time(b"16:43:52")
        );
        assert_eq!(
            Ok((
                &[][..],
                Time {
                    hour: 16,
                    minute: 43,
                    second: 0,
                    tz_offset: 0
                }
            )),
            time(b"16:43")
        );
    }
}
```

As you can see, even `do_parse!` parsers can be nested.
The sub-parts then must be mapped once for the inner parser and once into the final value of the outer parser.
`opt!` returns an `Option`. Either `None` if there is no input left or it applies the nested parser. If this parser doesn't fail, `Some(value)` is returned.

Our parser now works for simple time information.
But it leaves out one important bit: the timezone.

### Parsing the timezone: +0100

```
2015-07-02T19:45:00-0500
2015-07-02T19:45:00Z
2015-07-02T19:45:00+01
```

Above are three variants of valid dates with timezones.
The timezone in an ISO8601 string is either an appended `Z`, indicating UTC,
or it's separated using a sign (`+` or `-`) and appends the offset from UTC in hours and minutes (with the minutes being optional).

Let's cover the UTC special case first:

```rust
named!(timezone_utc <&[u8], i32>, map!(tag!("Z"), |_| 0));
```

This should look familiar by now.
It's a simple `Z` character, which we map to `0`.

The other case is the sign-separated hour and minute offset.

```rust
named!(timezone_hour <&[u8], i32>, do_parse!(
    sign: sign >>
    hour: hour >>
    minute: opt!(complete!(do_parse!(
        opt!(tag!(":")) >> minute: minute >> (minute)
    ))) >>
    ((sign * (hour as i32 * 3600 + minute.unwrap_or(0) as i32 * 60)))
));
```

We can re-use our already existing parsers and once again chain them to get what we want.
The minutes are optional (and might be separated using a colon).

Instead of keeping this as is, we're mapping it to the offset in seconds.
We will see why later.
We could also just map it to a tuple like <br>`(sign, hour, minute.unwrap_or(0))` and handle conversion at a later point.

Combined we get

```rust
named!(timezone <&[u8], i32>, alt!(timezone_utc | timezone_hour));
```

Putting this back into time we get:

```rust
named!(pub time <&[u8], Time>, do_parse!(
    hour: hour >>
    tag!(":") >>
    minute: minute >>
    second: opt!(complete!(do_parse!(
        tag!(":") >>
        second: second >>
        (second)
    ))) >>
    tz_offset: opt!(complete!(timezone)) >>
    (Time {hour, minute, second: second.unwrap_or(0), tz_offset: tz_offset.unwrap_or(0)})
));

#[cfg(test)]
mod tests {
    use time;
    use Time;
    #[test]
    fn parse_time_with_offset() {
        assert_eq!(
            Ok((
                &[][..],
                Time {
                    hour: 16,
                    minute: 43,
                    second: 52,
                    tz_offset: 0
                }
            )),
            time(b"16:43:52Z")
        );
        assert_eq!(
            Ok((
                &[][..],
                Time {
                    hour: 16,
                    minute: 43,
                    second: 0,
                    tz_offset: 5 * 3600
                }
            )),
            time(b"16:43+05")
        );
        assert_eq!(
            Ok((
                &[][..],
                Time {
                    hour: 16,
                    minute: 43,
                    second: 15,
                    tz_offset: 5 * 3600
                }
            )),
            time(b"16:43:15+0500")
        );

        assert_eq!(
            Ok((
                &[][..],
                Time {
                    hour: 16,
                    minute: 43,
                    second: 0,
                    tz_offset: -(5 * 3600 + 30 * 60)
                }
            )),
            time(b"16:43-05:30")
        );
    }
}

```

### Putting it all together

We now got individual parsers for the date, the time and the timezone offset.

Putting it all together, our final datetime parser looks quite small and easy to understand:

```rust
#[derive(Eq, PartialEq, Debug)]
pub struct DateTime {
    date: Date,
    time: Time,
}
named!(pub datetime <&[u8], DateTime>, do_parse!(
    date: date >>
    tag!("T") >>
    time: time >>
    (
        DateTime{
            date,
            time
        }
    )
));

#[cfg(test)]
mod tests {
    use datetime;
    use DateTime;

    #[test]
    fn parse_datetime() {
        assert_eq!(
            Ok((
                &[][..],
                DateTime {
                    date: Date {
                        year: 2007,
                        month: 08,
                        day: 31
                    },
                    time: Time {
                        hour: 16,
                        minute: 47,
                        second: 22,
                        tz_offset: 5 * 3600
                    }
                }
            )),
            datetime(b"2007-08-31T16:47:22+05:00")
        );
    }
}
```

Nothing special anymore. We can now parse all kinds of date strings:

```rust
datetime("2007-08-31T16:47+00:00");
datetime("2007-12-24T18:21Z");
datetime("2008-02-01T09:00:22+05");
```

But it will also parse invalid dates and times:

```rust
datetime("2234-13-42T25:70Z");
```

But this is fine for now. We can handle the actual validation in a later step.
For example, we could use [chrono][], a time library, [to handle this for us][chrono-convert].
Using chrono it's obvious why we already multiplied our timezone offset to be in seconds: this time we can just hand it off to chrono as is.

The full code for the previous version of this ISO8601 parser is available in [easy.rs][easy.rs]. The repository also includes [a more complex parser][lib.rs], that does some validation while parsing
(it checks that the time and date are reasonable values, but it does not check that it is a valid date for example)

### What's left?

These simple parsers or even some more complex ones are already usable.
At least if you already got all the data at hand and if a simple return value satisfies your needs.
But especially for larger and more complex formats like media files reading everything into memory and spitting out a single large value isn't sufficient at all.

nom is prepared for that.
Soon it will become as easy as using an object from which nom can [`Read`][read].
For most things you shouldn't worry about that, as a simple `BufReader` will work.

For the other end of the chain, nom has [Consumers][consumer].
A Consumer handles the complex part of actually requesting data, calling the right sub-parsers and holding the necessary state.
This is what you need to build yourself.
Internally it's best abstracted using some kind of state machine, so you always know which part of the format to expect next, how to parse it, what to return to the user and so on.
Take a look at [the MP4 parser][mp4], which has an `MP4Consumer` handling the different parts of the format.
Soon my own library, [rdb-rs][rdb-rs], will have this as well.

Small thing aside: Geoffroy created [machine][] to define a state machine and I got [microstate][] for this.

### Why am I doing this?

I'm currently developing [rdb-rs][rdb-rs], a library to parse and analyze Redis dump files.
It's currently limited to parsing and reformatting into several formats and can be mainly used as a CLI utility.
But [there are projects][rsedis] that could benefit from a nicer API to integrate it into another tool.
The current parser is hand-made. It's fast, it's working, but it provides a limited, not very extensible API.
I hope to get a proper parser done with nom, that I can build on to provide all necessary methods, while still being super-fast and memory-safe.
Work [already started][rdb-rs-nom], but I'm far from done for now

--

Thanks to [Geoffroy][gcouprie] for the discussions, the help and for reading a draft of this post.

[iso]: https://en.wikipedia.org/wiki/ISO_8601
[repo]: https://github.com/badboy/iso8601
[nom]: https://github.com/Geal/nom
[gcouprie]: https://twitter.com/gcouprie
[taken]: https://github.com/badboy/iso8601/blob/master/src/macros.rs#L20-L39
[rdb-rs]: http://rdb.fnordig.de/
[rsedis]: https://github.com/seppo0010/rsedis
[rdb-rs-nom]: https://github.com/badboy/rdb-rs/tree/nom-parser
[mp4]: https://github.com/Geal/nom/blob/master/tests/mp4.rs
[chrono]: https://crates.io/crates/chrono
[chrono-convert]: https://github.com/badboy/iso8601/blob/master/src/lib.rs#L65-L71
[easy.rs]: https://github.com/badboy/iso8601/blob/master/src/easy.rs
[lib.rs]: https://github.com/badboy/iso8601/blob/master/src/lib.rs
[consumer]: https://github.com/Geal/nom#consumers
[machine]: https://github.com/Geal/machine
[microstate]: https://github.com/badboy/microstate
[read]: http://doc.rust-lang.org/nightly/std/io/trait.Read.html
