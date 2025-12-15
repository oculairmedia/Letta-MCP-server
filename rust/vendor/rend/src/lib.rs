//! # rend
//!
//! rend provides cross-platform, endian-aware primitives for Rust.
//!
//! rend does not provide cross-platform alternatives for types that are
//! inherently cross-platform, such as `bool` and `u8`. It also does not provide
//! cross-platform alternatives for types that have an architecture-dependent
//! size, such as `isize` and `usize`. rend does not support custom types.
//!
//! rend is intended to be used to build portable types that can be shared
//! between different architectures.
//!
//! ## Features
//!
//! - `bytecheck`: Enables support for validating types using `bytecheck`.
//!
//! ## Crates
//!
//! - `zerocopy-0_8`
//!
//! ## Example:
#![doc = include_str!("../example.md")]
#![no_std]
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
#![cfg_attr(all(docsrs, not(doctest)), feature(doc_cfg))]

#[macro_use]
mod common;
#[cfg(feature = "bytecheck")]
mod context;
#[macro_use]
mod traits;
#[macro_use]
mod util;

pub mod unaligned;

#[cfg(target_has_atomic = "16")]
use core::sync::atomic::{AtomicI16, AtomicU16};
#[cfg(target_has_atomic = "32")]
use core::sync::atomic::{AtomicI32, AtomicU32};
#[cfg(target_has_atomic = "64")]
use core::sync::atomic::{AtomicI64, AtomicU64};
use core::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64,
    },
    sync::atomic::Ordering,
};

// `rustfmt` keeps changing the indentation of the attributes in this macro.
#[rustfmt::skip]
macro_rules! define_newtype {
    (
        $(#[$attr:meta])*
        $name:ident: $endian:ident $size_align:literal $prim:ty
    ) => {
        #[allow(non_camel_case_types)]
        #[doc = concat!(
            "A ",
            endian_name!($endian),
            "-endian `",
            stringify!($prim),
            "` with a guaranteed size and alignment of `",
            stringify!($size_align),
            "`.",
        )]
        $(#[$attr])*
        #[repr(C, align($size_align))]
        pub struct $name($prim);
    };
}

macro_rules! define_signed_integer {
    ($name:ident: $endian:ident $size_align:literal $prim:ident) => {
        define_newtype!(
            #[cfg_attr(
                feature = "zerocopy-0_8",
                derive(
                    zerocopy_derive::FromBytes,
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::Immutable,
                    zerocopy_derive::KnownLayout,
                ),
            )]
            $name: $endian $size_align $prim
        );
        impl_integer!($name: $endian $prim);
        impl_signed_integer_traits!($name: $endian $prim);
    };
}

macro_rules! define_signed_integers {
    ($($le:ident $be:ident: $size_align:literal $prim:ident),* $(,)?) => {
        $(
            define_signed_integer!($le: little $size_align $prim);
            define_signed_integer!($be: big $size_align $prim);
        )*
    };
}

define_signed_integers! {
    i16_le i16_be: 2 i16,
    i32_le i32_be: 4 i32,
    i64_le i64_be: 8 i64,
    i128_le i128_be: 16 i128,
}

macro_rules! define_unsigned_integer {
    ($name:ident: $endian:ident $size_align:literal $prim:ident) => {
        define_newtype!(
            #[cfg_attr(
                feature = "zerocopy-0_8",
                derive(
                    zerocopy_derive::FromBytes,
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::Immutable,
                    zerocopy_derive::KnownLayout,
                ),
            )]
            $name: $endian $size_align $prim
        );
        impl_integer!($name: $endian $prim);
        impl_unsigned_integer_traits!($name: $endian $prim);
    }
}

macro_rules! define_unsigned_integers {
    ($($le:ident $be:ident: $size_align:literal $prim:ident),* $(,)?) => {
        $(
            define_unsigned_integer!($le: little $size_align $prim);
            define_unsigned_integer!($be: big $size_align $prim);
        )*
    };
}

define_unsigned_integers! {
    u16_le u16_be: 2 u16,
    u32_le u32_be: 4 u32,
    u64_le u64_be: 8 u64,
    u128_le u128_be: 16 u128,
}

macro_rules! define_float {
    (
        $name:ident:
        $endian:ident $size_align:literal $prim:ty as $prim_int:ty
    ) => {
        define_newtype!(
            #[cfg_attr(
                feature = "zerocopy-0_8",
                derive(
                    zerocopy_derive::FromBytes,
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::Immutable,
                    zerocopy_derive::KnownLayout,
                ),
            )]
            $name: $endian $size_align $prim
        );
        impl_float!($name: $endian $prim as $prim_int);
    };
}

macro_rules! define_floats {
    ($(
        $le:ident $be:ident:
        $size_align:literal $prim:ty as $prim_int:ty
    ),* $(,)?) => {
        $(
            define_float!($le: little $size_align $prim as $prim_int);
            define_float!($be: big $size_align $prim as $prim_int);
        )*
    };
}

define_floats! {
    f32_le f32_be: 4 f32 as u32,
    f64_le f64_be: 8 f64 as u64,
}

macro_rules! define_char {
    ($name:ident: $endian:ident) => {
        define_newtype!(
            #[cfg_attr(
                feature = "zerocopy-0_8",
                derive(
                    // The generated impl for `zerocopy::TryFromBytes` is overly
                    // permissive. The derive macro doesn't understand that even
                    // though this struct only contains a `u32`, it still has a
                    // restricted set of valid bit patterns. Because
                    // `zerocopy::TryFromBytes` has hidden, semver-breaking
                    // members, I can't write a manual impl. So no impl for you.
                    //
                    // zerocopy_derive::TryFromBytes,
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::Immutable,
                    zerocopy_derive::KnownLayout,
                ),
            )]
            $name: $endian 4 u32
        );
        impl_char!($name: $endian);
    };
}

define_char!(char_le: little);
define_char!(char_be: big);

macro_rules! define_nonzero {
    (
        $name:ident:
        $endian:ident $size_align:literal $prim:ty as $prim_int:ty
    ) => {
        define_newtype!(
            #[cfg_attr(
                feature = "zerocopy-0_8",
                derive(
                    zerocopy_derive::TryFromBytes,
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::Immutable,
                    zerocopy_derive::KnownLayout,
                ),
            )]
            $name: $endian $size_align $prim
        );
        impl_nonzero!($name: $endian $prim as $prim_int);

        #[cfg(feature = "bytecheck")]
        // SAFETY: `check_bytes` only returns `Ok` if `value` points to a valid
        // non-zero value, which is the only requirement for `NonZero` integers.
        unsafe impl<C> bytecheck::CheckBytes<C> for $name
        where
            C: bytecheck::rancor::Fallible + ?Sized,
            C::Error: bytecheck::rancor::Trace,
            $prim: bytecheck::CheckBytes<C>,
        {
            #[inline]
            unsafe fn check_bytes(
                value: *const Self,
                context: &mut C,
            ) -> Result<(), C::Error> {
                use bytecheck::rancor::ResultExt as _;

                // SAFETY: `value` points to a `Self`, which has the same size
                // as a `$prim` and is at least as aligned as one. Note that the
                // bit pattern for 0 is always the same regardless of
                // endianness.
                unsafe {
                    <$prim>::check_bytes(value.cast(), context)
                        .with_trace(|| $crate::context::ValueCheckContext {
                            inner_name: core::stringify!($prim),
                            outer_name: core::stringify!($name),
                        })
                }
            }
        }
    };
}

macro_rules! define_nonzeros {
    ($(
        $le:ident $be:ident:
        $size_align:literal $prim:ty as $prim_int:ty
    ),* $(,)?) => {
        $(
            define_nonzero!($le: little $size_align $prim as $prim_int);
            define_nonzero!($be: big $size_align $prim as $prim_int);
        )*
    }
}

define_nonzeros! {
    NonZeroI16_le NonZeroI16_be: 2 NonZeroI16 as i16,
    NonZeroI32_le NonZeroI32_be: 4 NonZeroI32 as i32,
    NonZeroI64_le NonZeroI64_be: 8 NonZeroI64 as i64,
    NonZeroI128_le NonZeroI128_be: 16 NonZeroI128 as i128,
    NonZeroU16_le NonZeroU16_be: 2 NonZeroU16 as u16,
    NonZeroU32_le NonZeroU32_be: 4 NonZeroU32 as u32,
    NonZeroU64_le NonZeroU64_be: 8 NonZeroU64 as u64,
    NonZeroU128_le NonZeroU128_be: 16 NonZeroU128 as u128,
}

#[allow(dead_code)]
const fn fetch_ordering(order: Ordering) -> Ordering {
    match order {
        Ordering::Relaxed => Ordering::Relaxed,
        Ordering::Release => Ordering::Relaxed,
        Ordering::Acquire => Ordering::Acquire,
        Ordering::AcqRel => Ordering::Acquire,
        Ordering::SeqCst => Ordering::SeqCst,
        order => order,
    }
}

#[cfg(any(
    target_has_atomic = "16",
    target_has_atomic = "32",
    target_has_atomic = "64",
))]
macro_rules! define_atomic {
    (
        $name:ident:
        $endian:ident $size_align:literal $prim:ty as $prim_int:ty
    ) => {
        define_newtype!(
            #[cfg_attr(
                feature = "zerocopy-0_8",
                derive(
                    zerocopy_derive::FromBytes,
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::KnownLayout,
                ),
            )]
            $name: $endian $size_align $prim
        );

        impl $name {
            #[doc = concat!(
                "Returns a `",
                stringify!($name),
                "` containing `value`.",
            )]
            #[inline]
            pub const fn new(value: $prim_int) -> Self {
                Self(<$prim>::new(swap_endian!($endian value)))
            }
        }

        // SAFETY: An impl of `CheckBytes` with a `check_bytes` function that is
        // a no-op is sound for atomic integers.
        unsafe_impl_check_bytes_noop!(for $name);

        impl $name {
            /// Stores a value into the atomic integer if the current value is
            /// the same as the `current` value.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::compare_exchange`] for more information.",
            )]
            #[inline]
            pub fn compare_exchange(
                &self,
                current: $prim_int,
                new: $prim_int,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$prim_int, $prim_int> {
                match self.0.compare_exchange(
                    swap_endian!($endian current),
                    swap_endian!($endian new),
                    success,
                    failure,
                ) {
                    Ok(x) => Ok(swap_endian!($endian x)),
                    Err(x) => Err(swap_endian!($endian x)),
                }
            }

            /// Stores a value into the atomic integer if the current value is
            /// the same as the `current` value.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::compare_exchange_weak`] for more information.",
            )]
            #[inline]
            pub fn compare_exchange_weak(
                &self,
                current: $prim_int,
                new: $prim_int,
                success: Ordering,
                failure: Ordering,
            ) -> Result<$prim_int, $prim_int> {
                match self.0.compare_exchange_weak(
                    swap_endian!($endian current),
                    swap_endian!($endian new),
                    success,
                    failure,
                ) {
                    Ok(x) => Ok(swap_endian!($endian x)),
                    Err(x) => Ok(swap_endian!($endian x)),
                }
            }

            /// Adds to the current value, returning the previous value.
            ///
            #[doc = concat!(
                "Because addition is not an endian-agnostic operation, ",
                "`fetch_add` is implemented in terms of [`",
                stringify!($prim),
                "::compare_exchange_weak`] on ",
                opposite_endian_name!($endian),
                "-endian targets. This may result in worse performance on ",
                "those targets.",
            )]
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::fetch_add`] for more information.",
            )]
            #[inline]
            pub fn fetch_add(
                &self,
                val: $prim_int,
                order: Ordering,
            ) -> $prim_int {
                if_native_endian!(
                    $endian
                    self.0.fetch_add(val, order),
                    self.fetch_update_fast(
                        order,
                        fetch_ordering(order),
                        |x| x + val,
                    ),
                )
            }

            /// Bitwise "and" with the current value.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::fetch_and`] for more information.",
            )]
            #[inline]
            pub fn fetch_and(
                &self,
                val: $prim_int,
                order: Ordering,
            ) -> $prim_int {
                let val = swap_endian!($endian val);
                swap_endian!($endian self.0.fetch_and(val, order))
            }

            /// Maximum with the current value.
            ///
            #[doc = concat!(
                "Because maximum is not an endian-agnostic operation, ",
                "`fetch_max` is implemented in terms of [`",
                stringify!($prim),
                "::compare_exchange_weak`] on ",
                opposite_endian_name!($endian),
                "-endian targets. This may result in worse performance on ",
                "those targets.",
            )]
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::fetch_max`] for more information.",
            )]
            #[inline]
            pub fn fetch_max(
                &self,
                val: $prim_int,
                order: Ordering,
            ) -> $prim_int {
                if_native_endian!(
                    $endian
                    self.0.fetch_max(val, order),
                    self.fetch_update_fast(
                        order,
                        fetch_ordering(order),
                        |x| <$prim_int>::max(x, val),
                    ),
                )
            }

            /// Minimum with the current value.
            ///
            #[doc = concat!(
                "Because minimum is not an endian-agnostic operation, ",
                "`fetch_min` is implemented in terms of [`",
                stringify!($prim),
                "::compare_exchange_weak`] on ",
                opposite_endian_name!($endian),
                "-endian targets. This may result in worse performance on ",
                "those targets.",
            )]
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::fetch_min`] for more information.",
            )]
            #[inline]
            pub fn fetch_min(
                &self,
                val: $prim_int,
                order: Ordering,
            ) -> $prim_int {
                if_native_endian!(
                    $endian
                    self.0.fetch_min(val, order),
                    self.fetch_update_fast(
                        order,
                        fetch_ordering(order),
                        |x| <$prim_int>::min(x, val),
                    ),
                )
            }

            /// Bitwise "nand" with the current value.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::fetch_nand`] for more information.",
            )]
            #[inline]
            pub fn fetch_nand(
                &self,
                val: $prim_int,
                order: Ordering,
            ) -> $prim_int {
                let val = swap_endian!($endian val);
                swap_endian!($endian self.0.fetch_nand(val, order))
            }

            /// Bitwise "or" with the current value.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::fetch_or`] for more information.",
            )]
            #[inline]
            pub fn fetch_or(
                &self,
                val: $prim_int,
                order: Ordering,
            ) -> $prim_int {
                let val = swap_endian!($endian val);
                swap_endian!($endian self.0.fetch_or(val, order))
            }

            /// Subtracts from the current value, returning the previous value.
            ///
            #[doc = concat!(
                "Because subtraction is not an endian-agnostic operation, ",
                "`fetch_sub` is implemented in terms of [`",
                stringify!($prim),
                "::compare_exchange_weak`] on ",
                opposite_endian_name!($endian),
                "-endian targets. This may result in worse performance on ",
                "those targets.",
            )]
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::fetch_sub`] for more information.",
            )]
            #[inline]
            pub fn fetch_sub(
                &self,
                val: $prim_int,
                order: Ordering,
            ) -> $prim_int {
                if_native_endian!(
                    $endian
                    self.0.fetch_sub(val, order),
                    self.fetch_update_fast(
                        order,
                        fetch_ordering(order),
                        |x| x - val,
                    ),
                )
            }

            #[allow(dead_code)]
            #[inline(always)]
            fn fetch_update_fast<F: Fn($prim_int) -> $prim_int>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                f: F,
            ) -> $prim_int {
                let mut prev = swap_endian!($endian self.0.load(fetch_order));
                loop {
                    let next = swap_endian!($endian f(prev));
                    match self.0.compare_exchange_weak(
                        prev,
                        next,
                        set_order,
                        fetch_order,
                    ) {
                        Ok(x) => break x,
                        Err(next_prev) => {
                            prev = swap_endian!($endian next_prev);
                        }
                    }
                }
            }

            /// Fetches the value, and applies a function to it that returns an
            /// optional new value. Returns a `Result` of `Ok(previous_value)`
            /// if the function returned `Some(_)`, else `Err(previous_value)`.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::fetch_update`] for more information.",
            )]
            #[inline]
            pub fn fetch_update<F: FnMut($prim_int) -> Option<$prim_int>>(
                &self,
                set_order: Ordering,
                fetch_order: Ordering,
                mut f: F,
            ) -> Result<$prim_int, $prim_int> {
                self.0.fetch_update(set_order, fetch_order, |x| {
                    f(swap_endian!($endian x)).map(|y| swap_endian!($endian y))
                })
            }

            /// Bitwise "xor" with the current value.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::fetch_xor`] for more information.",
            )]
            #[inline]
            pub fn fetch_xor(
                &self,
                val: $prim_int,
                order: Ordering,
            ) -> $prim_int {
                let val = swap_endian!($endian val);
                swap_endian!($endian self.0.fetch_xor(val, order))
            }

            /// Consumes the atomic and returns the contained value.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::into_inner`] for more information.",
            )]
            #[inline]
            pub fn into_inner(self) -> $prim_int {
                swap_endian!($endian self.0.into_inner())
            }

            /// Loads a value from the atomic integer.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::load`] for more information.",
            )]
            #[inline]
            pub fn load(&self, order: Ordering) -> $prim_int {
                swap_endian!($endian self.0.load(order))
            }

            /// Stores a value into the atomic integer.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::store`] for more information.",
            )]
            #[inline]
            pub fn store(&self, val: $prim_int, order: Ordering) {
                self.0.store(swap_endian!($endian val), order);
            }

            /// Stores a value into the atomic integer, returning the previous
            /// value.
            ///
            #[doc = concat!(
                "See [`",
                stringify!($prim),
                "::swap`] for more information.",
            )]
            #[inline]
            pub fn swap(&self, val: $prim_int, order: Ordering) -> $prim_int {
                let val = swap_endian!($endian val);
                swap_endian!($endian self.0.swap(val, order))
            }
        }

        impl core::fmt::Debug for $name {
            #[inline]
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::fmt::Result {
                swap_endian!($endian self.load(Ordering::Relaxed)).fmt(f)
            }
        }

        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::new(<$prim_int>::default())
            }
        }

        impl From<$prim_int> for $name {
            #[inline]
            fn from(value: $prim_int) -> Self {
                Self::new(value)
            }
        }
    }
}

#[cfg(any(
    target_has_atomic = "16",
    target_has_atomic = "32",
    target_has_atomic = "64",
))]
macro_rules! define_atomics {
    ($(
        $le:ident $be:ident:
        $size_align:literal $prim:ty as $prim_int:ty
    ),* $(,)?) => {
        $(
            define_atomic!($le: little $size_align $prim as $prim_int);
            define_atomic!($be: big $size_align $prim as $prim_int);
        )*
    }
}

#[cfg(target_has_atomic = "16")]
define_atomics! {
    AtomicI16_le AtomicI16_be: 2 AtomicI16 as i16,
    AtomicU16_le AtomicU16_be: 2 AtomicU16 as u16,
}

#[cfg(target_has_atomic = "32")]
define_atomics! {
    AtomicI32_le AtomicI32_be: 4 AtomicI32 as i32,
    AtomicU32_le AtomicU32_be: 4 AtomicU32 as u32,
}

#[cfg(target_has_atomic = "64")]
define_atomics! {
    AtomicI64_le AtomicI64_be: 8 AtomicI64 as i64,
    AtomicU64_le AtomicU64_be: 8 AtomicU64 as u64,
}

#[cfg(test)]
mod tests {
    use core::mem::transmute;

    use super::*;

    #[test]
    fn signed_integers() {
        assert_size_align! {
            i16_be 2 2,
            i16_le 2 2,
            i32_be 4 4,
            i32_le 4 4,
            i64_be 8 8,
            i64_le 8 8,
            i128_be 16 16,
            i128_le 16 16,
        }

        unsafe {
            // i16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(i16_le::from_native(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(i16_be::from_native(0x0102)),
            );

            // i32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(i32_le::from_native(0x01020304)),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(i32_be::from_native(0x01020304)),
            );

            // i64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(i64_le::from_native(
                    0x0102030405060708
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(i64_be::from_native(
                    0x0102030405060708
                )),
            );

            // i128
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                    0x06, 0x05, 0x04, 0x03, 0x02, 0x01
                ],
                transmute::<_, [u8; 16]>(i128_le::from_native(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                    0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10
                ],
                transmute::<_, [u8; 16]>(i128_be::from_native(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
        }
    }

    #[test]
    fn unsigned_integers() {
        assert_size_align! {
            u16_be 2 2,
            u16_le 2 2,
            u32_be 4 4,
            u32_le 4 4,
            u64_be 8 8,
            u64_le 8 8,
            u128_be 16 16,
            u128_le 16 16,
        }

        unsafe {
            // u16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(u16_le::from_native(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(u16_be::from_native(0x0102)),
            );

            // u32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(u32_le::from_native(0x01020304)),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(u32_be::from_native(0x01020304)),
            );

            // u64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(u64_le::from_native(
                    0x0102030405060708
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(u64_be::from_native(
                    0x0102030405060708
                )),
            );

            // u128
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                    0x06, 0x05, 0x04, 0x03, 0x02, 0x01
                ],
                transmute::<_, [u8; 16]>(u128_le::from_native(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                    0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10
                ],
                transmute::<_, [u8; 16]>(u128_be::from_native(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
        }
    }

    #[test]
    fn floats() {
        assert_size_align! {
            f32_be 4 4,
            f32_le 4 4,
            f64_be 8 8,
            f64_le 8 8,
        }

        unsafe {
            // f32
            assert_eq!(
                [0xdb, 0x0f, 0x49, 0x40],
                transmute::<_, [u8; 4]>(f32_le::from_native(
                    core::f32::consts::PI
                )),
            );
            assert_eq!(
                [0x40, 0x49, 0x0f, 0xdb],
                transmute::<_, [u8; 4]>(f32_be::from_native(
                    core::f32::consts::PI
                )),
            );

            // f64
            assert_eq!(
                [0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40],
                transmute::<_, [u8; 8]>(f64_le::from_native(
                    core::f64::consts::PI
                )),
            );
            assert_eq!(
                [0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18],
                transmute::<_, [u8; 8]>(f64_be::from_native(
                    core::f64::consts::PI
                )),
            );

            // char
            assert_eq!(
                [0x89, 0xf3, 0x01, 0x00],
                transmute::<_, [u8; 4]>(char_le::from_native('🎉')),
            );
            assert_eq!(
                [0x00, 0x01, 0xf3, 0x89],
                transmute::<_, [u8; 4]>(char_be::from_native('🎉')),
            );
        }
    }

    #[test]
    fn signed_non_zero() {
        assert_size_align! {
            NonZeroI16_le 2 2,
            NonZeroI16_be 2 2,
            NonZeroI32_le 4 4,
            NonZeroI32_be 4 4,
            NonZeroI64_le 8 8,
            NonZeroI64_be 8 8,
            NonZeroI128_le 16 16,
            NonZeroI128_be 16 16,
        }

        unsafe {
            // NonZeroI16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(NonZeroI16_le::new_unchecked(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(NonZeroI16_be::new_unchecked(0x0102)),
            );

            // NonZeroI32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(NonZeroI32_le::new_unchecked(
                    0x01020304
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(NonZeroI32_be::new_unchecked(
                    0x01020304
                )),
            );

            // NonZeroI64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(NonZeroI64_le::new_unchecked(
                    0x0102030405060708
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(NonZeroI64_be::new_unchecked(
                    0x0102030405060708
                )),
            );

            // NonZeroI128
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                    0x06, 0x05, 0x04, 0x03, 0x02, 0x01
                ],
                transmute::<_, [u8; 16]>(NonZeroI128_le::new_unchecked(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                    0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10
                ],
                transmute::<_, [u8; 16]>(NonZeroI128_be::new_unchecked(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
        }
    }

    #[test]
    fn unsigned_non_zero() {
        assert_size_align! {
            NonZeroU16_le 2 2,
            NonZeroU16_be 2 2,
            NonZeroU32_le 4 4,
            NonZeroU32_be 4 4,
            NonZeroU64_le 8 8,
            NonZeroU64_be 8 8,
            NonZeroU128_le 16 16,
            NonZeroU128_be 16 16,
        }

        unsafe {
            // NonZeroU16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(NonZeroU16_le::new_unchecked(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(NonZeroU16_be::new_unchecked(0x0102)),
            );

            // NonZeroU32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(NonZeroU32_le::new_unchecked(
                    0x01020304
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(NonZeroU32_be::new_unchecked(
                    0x01020304
                )),
            );

            // NonZeroU64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(NonZeroU64_le::new_unchecked(
                    0x0102030405060708
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(NonZeroU64_be::new_unchecked(
                    0x0102030405060708
                )),
            );

            // NonZeroU128
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                    0x06, 0x05, 0x04, 0x03, 0x02, 0x01
                ],
                transmute::<_, [u8; 16]>(NonZeroU128_le::new_unchecked(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                    0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10
                ],
                transmute::<_, [u8; 16]>(NonZeroU128_be::new_unchecked(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
        }
    }

    #[cfg(feature = "bytecheck")]
    #[test]
    fn unaligned_non_zero() {
        use bytecheck::{
            rancor::{Failure, Strategy},
            CheckBytes,
        };
        use unaligned::{u32_ule, NonZeroU32_ule};

        let zero = u32_ule::from_native(0);
        let ptr = (&zero as *const u32_ule).cast::<NonZeroU32_ule>();
        let mut unit = ();
        let context = Strategy::<_, Failure>::wrap(&mut unit);
        unsafe {
            <NonZeroU32_ule as CheckBytes<Strategy<(), Failure>>>::check_bytes(
                ptr, context,
            )
            .unwrap_err();
        }
    }

    #[cfg(target_has_atomic = "16")]
    #[test]
    fn atomics_16() {
        assert_size_align! {
            AtomicI16_le 2 2,
            AtomicI16_be 2 2,
            AtomicU16_le 2 2,
            AtomicU16_be 2 2,
        }

        unsafe {
            // AtomicI16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(AtomicI16_le::new(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(AtomicI16_be::new(0x0102)),
            );

            // AtomicU16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(AtomicU16_le::new(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(AtomicU16_be::new(0x0102)),
            );
        }
    }

    #[cfg(target_has_atomic = "32")]
    #[test]
    fn atomics_32() {
        assert_size_align! {
            AtomicI32_le 4 4,
            AtomicI32_be 4 4,
            AtomicU32_le 4 4,
            AtomicU32_be 4 4,
        }

        unsafe {
            // AtomicI32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(AtomicI32_le::new(0x01020304)),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(AtomicI32_be::new(0x01020304)),
            );

            // AtomicU32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(AtomicU32_le::new(0x01020304)),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(AtomicU32_be::new(0x01020304)),
            );
        }
    }

    #[cfg(target_has_atomic = "64")]
    #[test]
    fn atomics_64() {
        assert_size_align! {
            AtomicI64_le 8 8,
            AtomicI64_be 8 8,
            AtomicU64_le 8 8,
            AtomicU64_be 8 8,
        }

        unsafe {
            // AtomicI64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(AtomicI64_le::new(0x0102030405060708)),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(AtomicI64_be::new(0x0102030405060708)),
            );

            // AtomicU64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(AtomicU64_le::new(0x0102030405060708)),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(AtomicU64_be::new(0x0102030405060708)),
            );
        }
    }
}
