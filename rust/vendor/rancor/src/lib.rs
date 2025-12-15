//! # rancor
//!
//! rancor provides scalable and efficient error handling without using type
//! composition. This makes it best-suited for situations where:
//!
//! - Programmatic error introspection is not useful
//! - Functions may error, but succeed most of the time
//! - Errors should provide as much useful detail as possible when emitted
//! - Use cases include both `no_std` and targets with support for `std`
//!
//! ## Features
//!
//! - `alloc`: Provides the [`BoxedError`] type. Enabled by default.

#![deny(
    future_incompatible,
    missing_docs,
    nonstandard_style,
    unsafe_op_in_unsafe_fn,
    unused,
    warnings,
    clippy::all,
    clippy::missing_safety_doc,
    clippy::undocumented_unsafe_blocks,
    rustdoc::broken_intra_doc_links,
    rustdoc::missing_crate_level_docs
)]
#![no_std]
#![cfg_attr(all(docsrs, not(doctest)), feature(doc_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
mod boxed_error;
#[cfg(feature = "alloc")]
mod thin_box;

use core::{
    error, fmt,
    hint::unreachable_unchecked,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

/// An error type which can add additional "trace" information to itself.
///
/// Some functions only add additional context to errors created by other
/// functions, rather than creating errors themselves. With generics, it's
/// therefore possible to have a generic function which can produce errors with
/// some type arguments but not with others. In these cases, `Trace` allows
/// those functions to add context if an error can occur, and compile out the
/// context if the error type is [`Infallible`] or [`Panic`].
///
/// # Example
///
/// ```
/// use rancor::{ResultExt, Trace};
///
/// trait Print<E> {
///     fn print(&self, message: &str) -> Result<(), E>;
/// }
///
/// fn print_hello_world<T: Print<E>, E: Trace>(printer: &T) -> Result<(), E> {
///     printer.print("hello").trace("failed to print hello")?;
///     printer.print("world").trace("failed to print world")?;
///     Ok(())
/// }
/// ```
pub trait Trace: Sized + Send + Sync + 'static {
    /// Adds an additional trace to this error, returning a new error.
    fn trace<R>(self, trace: R) -> Self
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static;
}

/// An error type which can be uniformly constructed from an [`Error`] and
/// additional trace information.
///
/// # Example
///
/// ```
/// use core::{error::Error, fmt};
///
/// use rancor::{fail, Source};
///
/// #[derive(Debug)]
/// struct DivideByZeroError;
///
/// impl fmt::Display for DivideByZeroError {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "attempted to divide by zero")
///     }
/// }
///
/// impl Error for DivideByZeroError {}
///
/// fn try_divide<E: Source>(a: i32, b: i32) -> Result<i32, E> {
///     if b == 0 {
///         fail!(DivideByZeroError);
///     }
///     Ok(a / b)
/// }
/// ```
pub trait Source: Trace + error::Error {
    /// Returns a new `Self` using the given [`Error`].
    ///
    /// Depending on the specific implementation, this may box the error,
    /// immediately emit a diagnostic, or discard it and only remember that some
    /// error occurred.
    fn new<T: error::Error + Send + Sync + 'static>(source: T) -> Self;
}

/// A type with fallible operations that return its associated error type.
///
/// `Fallible` turns an error type parameter into an associated type of another
/// parameter. You can equip an existing type with a `Fallible` implementation
/// by wrapping it in a [`Strategy`].
///
/// # Example
///
/// ```
/// use rancor::{Failure, Fallible, Strategy};
///
/// trait Operator<E = <Self as Fallible>::Error> {
///     fn operate(&self, lhs: i32, rhs: i32) -> Result<i32, E>;
/// }
///
/// impl<T: Operator<E> + ?Sized, E> Operator<E> for Strategy<T, E> {
///     fn operate(&self, lhs: i32, rhs: i32) -> Result<i32, E> {
///         T::operate(self, lhs, rhs)
///     }
/// }
///
/// struct Add;
///
/// impl<E> Operator<E> for Add {
///     fn operate(&self, lhs: i32, rhs: i32) -> Result<i32, E> {
///         Ok(lhs + rhs)
///     }
/// }
///
/// fn operate_one_one<T: Operator + Fallible>(
///     operator: &T,
/// ) -> Result<i32, T::Error> {
///     operator.operate(1, 1)
/// }
///
/// assert_eq!(
///     operate_one_one(Strategy::<_, Failure>::wrap(&mut Add)),
///     Ok(2)
/// );
/// ```
pub trait Fallible {
    /// The error type associated with this type's operations.
    type Error;
}

/// Equips a type with a `Fallible` implementation that chooses `E` as its error
/// type.
///
/// # Example
///
/// ```
/// use rancor::{Failure, Fallible, Strategy};
///
/// trait Print<E = <Self as Fallible>::Error> {
///     fn print(&self, message: &str) -> Result<(), E>;
/// }
///
/// impl<T: Print<E> + ?Sized, E> Print<E> for Strategy<T, E> {
///     fn print(&self, message: &str) -> Result<(), E> {
///         T::print(self, message)
///     }
/// }
///
/// struct StdOut;
///
/// impl<E> Print<E> for StdOut {
///     fn print(&self, message: &str) -> Result<(), E> {
///         println!("{message}");
///         Ok(())
///     }
/// }
///
/// Strategy::<_, Failure>::wrap(&mut StdOut)
///     .print("hello world")
///     .unwrap();
/// ```
#[repr(transparent)]
pub struct Strategy<T: ?Sized, E> {
    _error: PhantomData<E>,
    inner: T,
}

impl<T: ?Sized, E> Fallible for Strategy<T, E> {
    type Error = E;
}

impl<T: ?Sized, E> Strategy<T, E> {
    /// Wraps the given mutable reference, returning a mutable reference to a
    /// `Strategy`.
    ///
    /// ## Example
    /// ```
    /// use core::ops::Deref;
    ///
    /// use rancor::{Failure, Strategy};
    /// fn test() {
    ///     struct Inner {
    ///         value: u64,
    ///     }
    ///
    ///     let mut inner = Inner { value: 10 };
    ///
    ///     let inner_value_ptr = &inner.value as *const u64;
    ///     let strategy: &mut Strategy<Inner, Failure> =
    ///         Strategy::wrap(&mut inner);
    ///     let strategy_value_ptr = (&strategy.deref().value) as *const u64;
    ///     assert_eq!(inner_value_ptr, strategy_value_ptr);
    ///     // Strategy wraps a type but does not change its memory layout.
    /// }
    ///
    /// test();
    /// ```
    pub fn wrap(inner: &mut T) -> &mut Self {
        // SAFETY: `Strategy` is `repr(transparent)` and so has the same layout
        // as `T`. The input and output lifetimes are the same, so mutable
        // aliasing rules will be upheld. Finally, because the inner `T` is the
        // final element of `Strategy`, the pointer metadata of the two pointers
        // will be the same.
        unsafe { core::mem::transmute::<&mut T, &mut Self>(inner) }
    }
}

impl<T: ?Sized, E> Deref for Strategy<T, E> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ?Sized, E> DerefMut for Strategy<T, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Returns the given error from this function.
///
/// The current function must return a `Result<_, E>` where `E` implements
/// [`Source`].
///
/// # Example
///
/// ```
/// use core::{error::Error, fmt};
///
/// use rancor::{fail, Source};
///
/// #[derive(Debug)]
/// struct DivideByZeroError;
///
/// impl fmt::Display for DivideByZeroError {
///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///         write!(f, "attempted to divide by zero")
///     }
/// }
///
/// impl Error for DivideByZeroError {}
///
/// fn divide<E: Source>(a: i32, b: i32) -> Result<i32, E> {
///     if b == 0 {
///         fail!(DivideByZeroError);
///     }
///     Ok(a / b)
/// }
/// ```
#[macro_export]
macro_rules! fail {
    ($($x:tt)*) => {
        return ::core::result::Result::Err($crate::Source::new($($x)*));
    };
}

/// Helper methods for `Result`s.
pub trait ResultExt<T, E> {
    /// Returns a `Result` with this error type converted to `U`.
    ///
    /// # Example
    ///
    /// ```
    /// use rancor::{Failure, ResultExt as _};
    ///
    /// let result = "1_000".parse::<i32>().into_error::<Failure>();
    /// ```
    fn into_error<U>(self) -> Result<T, U>
    where
        U: Source,
        E: error::Error + Send + Sync + 'static;

    /// Returns a `Result` with this error type converted to `U` and with an
    /// additional `trace` message added.
    ///
    /// # Example
    ///
    /// ```
    /// use rancor::{BoxedError, ResultExt as _};
    ///
    /// let result = "1_000"
    ///     .parse::<i32>()
    ///     .into_trace::<BoxedError, _>("while parsing 1_000");
    /// ```
    fn into_trace<U, R>(self, trace: R) -> Result<T, U>
    where
        U: Source,
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        E: error::Error + Send + Sync + 'static;

    /// Returns a `Result` with this error type converted to `U` and with an
    /// additional trace message added by evaluating the given function `f`. The
    /// function is evaluated only if an error occurred.
    ///
    /// # Example
    ///
    /// ```
    /// use rancor::{BoxedError, ResultExt as _};
    ///
    /// let input = "1_000";
    /// let result =
    ///     input
    ///         .parse::<i32>()
    ///         .into_with_trace::<BoxedError, _, _>(|| {
    ///             format!("while parsing {input}")
    ///         });
    /// ```
    fn into_with_trace<U, R, F>(self, f: F) -> Result<T, U>
    where
        U: Source,
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> R,
        E: error::Error + Send + Sync + 'static;

    /// Adds an additional `trace` message to the error value of this type.
    ///
    /// # Example
    ///
    /// ```
    /// use rancor::{BoxedError, ResultExt as _};
    ///
    /// let result = "1_000"
    ///     .parse::<i32>()
    ///     .into_error::<BoxedError>()
    ///     .trace("while parsing 1_000");
    /// ```
    fn trace<R>(self, trace: R) -> Result<T, E>
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        E: Trace;

    /// Adds an additional trace message to the error value of this type by
    /// evaluating the given function `f`. The function is evaluated only if an
    /// error occurred.
    ///
    /// # Example
    ///
    /// ```
    /// use rancor::{BoxedError, ResultExt as _};
    ///
    /// let input = "1_000";
    /// let result = input
    ///     .parse::<i32>()
    ///     .into_error::<BoxedError>()
    ///     .with_trace(|| format!("while parsing {input}"));
    /// ```
    fn with_trace<R, F>(self, f: F) -> Result<T, E>
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> R,
        E: Trace;

    /// Safely unwraps a result that is always `Ok`.
    ///
    /// In order to call this method, the error type of this `Result` must be a
    /// [`Never`] type.
    ///
    /// # Example
    ///
    /// ```
    /// use rancor::{Infallible, ResultExt};
    ///
    /// let inner = Ok::<i32, Infallible>(10).always_ok();
    /// ```
    fn always_ok(self) -> T
    where
        E: Never;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn into_error<U>(self) -> Result<T, U>
    where
        U: Source,
        E: error::Error + Send + Sync + 'static,
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(U::new(e)),
        }
    }

    fn into_trace<U, R>(self, trace: R) -> Result<T, U>
    where
        U: Source,
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        E: error::Error + Send + Sync + 'static,
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(U::new(e).trace(trace)),
        }
    }

    fn into_with_trace<U, R, F>(self, f: F) -> Result<T, U>
    where
        U: Source,
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> R,
        E: error::Error + Send + Sync + 'static,
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(U::new(e).trace(f())),
        }
    }

    fn trace<R>(self, trace: R) -> Result<T, E>
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        E: Trace,
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(e.trace(trace)),
        }
    }

    fn with_trace<R, F>(self, f: F) -> Result<T, E>
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> R,
        E: Trace,
    {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(e.trace(f())),
        }
    }

    fn always_ok(self) -> T
    where
        E: Never,
    {
        match self {
            Ok(x) => x,
            Err(e) => unreachable_checked(e),
        }
    }
}

/// Helper methods for `Option`s.
pub trait OptionExt<T> {
    /// Returns a `Result` with an error indicating that `Some` was expected but
    /// `None` was found.
    ///
    /// # Example
    ///
    /// ```
    /// use rancor::{Failure, OptionExt};
    ///
    /// let result = Some(10).into_error::<Failure>();
    /// ```
    fn into_error<E>(self) -> Result<T, E>
    where
        E: Source;

    /// Returns a `Result` with an error indicating that `Some` was expected but
    /// `None` was found, and with an additional `trace` message added.
    ///
    /// # Example
    ///
    /// ```
    /// use rancor::{Failure, OptionExt};
    ///
    /// ##[rustfmt::skip]
    /// let result = Some(10).
    ///     into_trace::<Failure, _>("while converting Some(10)");
    /// ```
    fn into_trace<E, R>(self, trace: R) -> Result<T, E>
    where
        E: Source,
        R: fmt::Debug + fmt::Display + Send + Sync + 'static;

    /// Returns a `Result` with an error indicating that `Some` was expected but
    /// `None` was found, and with an additional trace message added by
    /// evaluating the given function `f`. The function is evaluated only if an
    /// error occurred.
    ///
    /// # Example
    ///
    /// ```
    /// use rancor::{Failure, OptionExt};
    ///
    /// let input = Some(10);
    /// let result = input.into_with_trace::<Failure, _, _>(|| {
    ///     format!("while converting {input:?}")
    /// });
    /// ```
    fn into_with_trace<E, R, F>(self, f: F) -> Result<T, E>
    where
        E: Source,
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> R;
}

/// A type that can never be produced.
///
/// Never types include the unstable `!` type, enums with no variants, or any
/// type that always contains a never type (e.g. a struct with a `Never` field).
///
/// # Safety
///
/// It must be impossible to produce a value of this type.
pub unsafe trait Never {}

/// Consumes a `Never` type, returning a primitive `!`.
///
/// This is a safe version of [`unreachable_unchecked`] for `Never` types.
///
/// # Example
///
/// ```
/// use rancor::{unreachable_checked, Infallible};
///
/// let result = Ok::<i32, Infallible>(10);
/// match result {
///     Ok(i) => println!("i"),
///     Err(e) => unreachable_checked(e),
/// }
/// ```
#[inline(always)]
pub const fn unreachable_checked<T: Never>(_: T) -> ! {
    // SAFETY: Types that implement `Never` cannot be constructed,
    // so this is unreachable.
    unsafe { unreachable_unchecked() }
}

#[derive(Debug)]
struct NoneError;

impl fmt::Display for NoneError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "`Option` is `None`, expected `Some`")
    }
}

impl error::Error for NoneError {}

impl<T> OptionExt<T> for Option<T> {
    fn into_error<E>(self) -> Result<T, E>
    where
        E: Source,
    {
        match self {
            Some(x) => Ok(x),
            None => Err(E::new(NoneError)),
        }
    }

    fn into_trace<E, R>(self, trace: R) -> Result<T, E>
    where
        E: Source,
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
    {
        match self {
            Some(x) => Ok(x),
            None => Err(E::new(NoneError).trace(trace)),
        }
    }

    fn into_with_trace<E, R, F>(self, f: F) -> Result<T, E>
    where
        E: Source,
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> R,
    {
        match self {
            Some(x) => Ok(x),
            None => Err(E::new(NoneError).trace(f())),
        }
    }
}

/// A re-export of `core::convert::Infallible`.
pub use core::convert::Infallible;

// SAFETY: `Infallible` is an enum with no variants, and so cannot be produced.
unsafe impl Never for Infallible {}

impl Trace for Infallible {
    fn trace<R>(self, _: R) -> Self
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
    {
        match self {}
    }
}

/// An error type that does not occupy any space, panicking on creation instead.
///
/// Because panicking occurs immediately upon creation, this error type will not
/// print any additional trace information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Panic {}

// SAFETY: `Panic` is an enum with no variants, and so cannot be produced.
unsafe impl Never for Panic {}

impl fmt::Display for Panic {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

impl error::Error for Panic {}

impl Trace for Panic {
    fn trace<R>(self, _: R) -> Self
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
    {
        match self {}
    }
}

impl Source for Panic {
    fn new<T: fmt::Display>(error: T) -> Self {
        panic!("created a new `Panic` from: {error}");
    }
}

/// An error type that only preserves success or failure, throwing away any more
/// detailed error messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Failure;

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed without error information")
    }
}

impl error::Error for Failure {}

impl Trace for Failure {
    fn trace<R>(self, _: R) -> Self
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
    {
        self
    }
}

impl Source for Failure {
    fn new<T: error::Error + Send + Sync + 'static>(_: T) -> Self {
        Self
    }
}

#[cfg(feature = "alloc")]
pub use boxed_error::BoxedError;

#[cfg(all(debug_assertions, feature = "alloc"))]
type ErrorType = BoxedError;
#[cfg(not(all(debug_assertions, feature = "alloc")))]
type ErrorType = Failure;

/// A good general-purpose error type.
///
/// If `debug_assertions` and the `alloc` feature are enabled, then this error
/// will have the same behavior as [`BoxedError`]. Otherwise, it will behave
/// like [`Failure`].
#[derive(Debug)]
pub struct Error {
    inner: ErrorType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)?;
        #[cfg(not(all(debug_assertions, feature = "alloc")))]
        write!(
            f,
            "; enable debug assertions and the `alloc` feature in rancor for \
             error information"
        )?;

        Ok(())
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.inner.source()
    }
}

impl Trace for Error {
    fn trace<R>(self, trace: R) -> Self
    where
        R: fmt::Debug + fmt::Display + Send + Sync + 'static,
    {
        Self {
            inner: self.inner.trace(trace),
        }
    }
}

impl Source for Error {
    fn new<T: error::Error + Send + Sync + 'static>(source: T) -> Self {
        Self {
            inner: ErrorType::new(source),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Inner {
        value: u64,
    }

    #[test]
    fn test_strategy() {
        let mut inner = Inner { value: 10 };
        let address = &inner.value as *const u64;
        let strategy: &mut Strategy<Inner, Failure> =
            Strategy::wrap(&mut inner);
        let s_address = (&strategy.inner.value) as *const u64;
        assert_eq!(address, s_address);

        assert_eq!(strategy.value, 10);
        strategy.value = 20;
        assert_eq!(strategy.value, 20);
        assert_eq!(inner.value, 20);
    }
}
