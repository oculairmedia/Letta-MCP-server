//! Cross-platform primitives with unaligned representations.

use core::{
    concat,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64,
    },
};

// `rustfmt` keeps changing the indentation of the attributes in this macro.
#[rustfmt::skip]
macro_rules! define_unaligned_newtype {
    (
        $(#[$attr:meta])*
        $name:ident: $endian:ident $size:literal $prim:ty
    ) => {
        #[allow(non_camel_case_types)]
        #[doc = concat!(
            "A ",
            endian_name!($endian),
            "-endian unaligned `",
            stringify!($prim),
            "` with a guaranteed size of `",
            stringify!($size),
            "` and alignment of `1`.",
        )]
        $(#[$attr])*
        #[repr(C, packed)]
        pub struct $name($prim);
    };
}

macro_rules! define_unaligned_signed_integer {
    ($name:ident: $endian:ident $size:literal $prim:ident) => {
        define_unaligned_newtype!(
            #[cfg_attr(
                feature = "zerocopy-0_8",
                derive(
                    zerocopy_derive::FromBytes,
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::Immutable,
                    zerocopy_derive::KnownLayout,
                    zerocopy_derive::Unaligned,
                ),
            )]
            $name: $endian $size $prim
        );
        impl_integer!($name: $endian $prim);
        impl_signed_integer_traits!($name: $endian $prim);
    };
}

macro_rules! define_unaligned_signed_integers {
    ($($le:ident $be:ident: $size:literal $prim:ident),* $(,)?) => {
        $(
            define_unaligned_signed_integer!($le: little $size $prim);
            define_unaligned_signed_integer!($be: big $size $prim);
        )*
    };
}

define_unaligned_signed_integers! {
    i16_ule i16_ube: 2 i16,
    i32_ule i32_ube: 4 i32,
    i64_ule i64_ube: 8 i64,
    i128_ule i128_ube: 16 i128,
}

macro_rules! define_unaligned_unsigned_integer {
    ($name:ident: $endian:ident $size:literal $prim:ident) => {
        define_unaligned_newtype!(
            #[cfg_attr(
                feature = "zerocopy-0_8",
                derive(
                    zerocopy_derive::FromBytes,
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::Immutable,
                    zerocopy_derive::KnownLayout,
                    zerocopy_derive::Unaligned,
                ),
            )]
            $name: $endian $size $prim
        );
        impl_integer!($name: $endian $prim);
        impl_unsigned_integer_traits!($name: $endian $prim);
    }
}

macro_rules! define_unaligned_unsigned_integers {
    ($($le:ident $be:ident: $size:literal $prim:ident),* $(,)?) => {
        $(
            define_unaligned_unsigned_integer!($le: little $size $prim);
            define_unaligned_unsigned_integer!($be: big $size $prim);
        )*
    };
}

define_unaligned_unsigned_integers! {
    u16_ule u16_ube: 2 u16,
    u32_ule u32_ube: 4 u32,
    u64_ule u64_ube: 8 u64,
    u128_ule u128_ube: 16 u128,
}

macro_rules! define_unaligned_float {
    ($name:ident: $endian:ident $size:literal $prim:ty as $prim_int:ty) => {
        define_unaligned_newtype!(
            #[cfg_attr(
                feature = "zerocopy-0_8",
                derive(
                    zerocopy_derive::FromBytes,
                    zerocopy_derive::IntoBytes,
                    zerocopy_derive::Immutable,
                    zerocopy_derive::KnownLayout,
                    zerocopy_derive::Unaligned,
                ),
            )]
            $name: $endian $size $prim
        );
        impl_float!($name: $endian $prim as $prim_int);
    };
}

macro_rules! define_unaligned_floats {
    ($(
        $le:ident $be:ident:
        $size:literal $prim:ty as $prim_int:ty
    ),* $(,)?) => {
        $(
            define_unaligned_float!($le: little $size $prim as $prim_int);
            define_unaligned_float!($be: big $size $prim as $prim_int);
        )*
    };
}

define_unaligned_floats! {
    f32_ule f32_ube: 4 f32 as u32,
    f64_ule f64_ube: 8 f64 as u64,
}

macro_rules! define_unaligned_char {
    ($name:ident: $endian:ident) => {
        define_unaligned_newtype!(
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
                    zerocopy_derive::Unaligned,
                ),
            )]
            $name: $endian 4 u32
        );
        impl_char!($name: $endian);
    };
}

define_unaligned_char!(char_ule: little);
define_unaligned_char!(char_ube: big);

macro_rules! define_unaligned_nonzero {
    ($name:ident: $endian:ident $size:literal $prim:ty as $prim_int:ty) => {
        define_unaligned_newtype!($name: $endian $size $prim);
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
                // as a `$prim_int` and which the caller has guaranteed is valid
                // for reads. All bit patterns are valid for `$prim_int`.
                let value = unsafe {
                    value.cast::<$prim_int>().read_unaligned()
                };
                let ptr = (&value as *const $prim_int).cast::<$prim>();
                // SAFETY: `ptr` points to a `$prim_int` and so is guaranteed to
                // be aligned and point to enough bytes to represent a `$prim`.
                unsafe {
                    <$prim>::check_bytes(ptr, context)
                        .with_trace(|| $crate::context::ValueCheckContext {
                            inner_name: core::stringify!($prim),
                            outer_name: core::stringify!($name),
                        })
                }
            }
        }
    };
}

macro_rules! define_unaligned_nonzeros {
    ($(
        $le:ident $be:ident:
        $size:literal $prim:ty as $prim_int:ty
    ),* $(,)?) => {
        $(
            define_unaligned_nonzero!($le: little $size $prim as $prim_int);
            define_unaligned_nonzero!($be: big $size $prim as $prim_int);
        )*
    }
}

define_unaligned_nonzeros! {
    NonZeroI16_ule NonZeroI16_ube: 2 NonZeroI16 as i16,
    NonZeroI32_ule NonZeroI32_ube: 2 NonZeroI32 as i32,
    NonZeroI64_ule NonZeroI64_ube: 4 NonZeroI64 as i64,
    NonZeroI128_ule NonZeroI128_ube: 4 NonZeroI128 as i128,
    NonZeroU16_ule NonZeroU16_ube: 8 NonZeroU16 as u16,
    NonZeroU32_ule NonZeroU32_ube: 8 NonZeroU32 as u32,
    NonZeroU64_ule NonZeroU64_ube: 16 NonZeroU64 as u64,
    NonZeroU128_ule NonZeroU128_ube: 16 NonZeroU128 as u128,
}

#[cfg(test)]
mod tests {
    use core::mem::transmute;

    use super::*;

    #[test]
    fn signed_integers() {
        assert_size_align! {
            i16_ube 2 1,
            i16_ule 2 1,
            i32_ube 4 1,
            i32_ule 4 1,
            i64_ube 8 1,
            i64_ule 8 1,
            i128_ube 16 1,
            i128_ule 16 1,
        }

        unsafe {
            // i16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(i16_ule::from_native(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(i16_ube::from_native(0x0102)),
            );

            // i32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(i32_ule::from_native(0x01020304)),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(i32_ube::from_native(0x01020304)),
            );

            // i64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(i64_ule::from_native(
                    0x0102030405060708
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(i64_ube::from_native(
                    0x0102030405060708
                )),
            );

            // i128
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                    0x06, 0x05, 0x04, 0x03, 0x02, 0x01
                ],
                transmute::<_, [u8; 16]>(i128_ule::from_native(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                    0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10
                ],
                transmute::<_, [u8; 16]>(i128_ube::from_native(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
        }
    }

    #[test]
    fn unsigned_integers() {
        assert_size_align! {
            u16_ube 2 1,
            u16_ule 2 1,
            u32_ube 4 1,
            u32_ule 4 1,
            u64_ube 8 1,
            u64_ule 8 1,
            u128_ube 16 1,
            u128_ule 16 1,
        }

        unsafe {
            // u16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(u16_ule::from_native(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(u16_ube::from_native(0x0102)),
            );

            // u32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(u32_ule::from_native(0x01020304)),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(u32_ube::from_native(0x01020304)),
            );

            // u64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(u64_ule::from_native(
                    0x0102030405060708
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(u64_ube::from_native(
                    0x0102030405060708
                )),
            );

            // u128
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                    0x06, 0x05, 0x04, 0x03, 0x02, 0x01
                ],
                transmute::<_, [u8; 16]>(u128_ule::from_native(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                    0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10
                ],
                transmute::<_, [u8; 16]>(u128_ube::from_native(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
        }
    }

    #[test]
    fn floats() {
        assert_size_align! {
            f32_ube 4 1,
            f32_ule 4 1,
            f64_ube 8 1,
            f64_ule 8 1,
        }

        unsafe {
            // f32
            assert_eq!(
                [0xdb, 0x0f, 0x49, 0x40],
                transmute::<_, [u8; 4]>(f32_ule::from_native(
                    core::f32::consts::PI
                )),
            );
            assert_eq!(
                [0x40, 0x49, 0x0f, 0xdb],
                transmute::<_, [u8; 4]>(f32_ube::from_native(
                    core::f32::consts::PI
                )),
            );

            // f64
            assert_eq!(
                [0x18, 0x2d, 0x44, 0x54, 0xfb, 0x21, 0x09, 0x40],
                transmute::<_, [u8; 8]>(f64_ule::from_native(
                    core::f64::consts::PI
                )),
            );
            assert_eq!(
                [0x40, 0x09, 0x21, 0xfb, 0x54, 0x44, 0x2d, 0x18],
                transmute::<_, [u8; 8]>(f64_ube::from_native(
                    core::f64::consts::PI
                )),
            );

            // char
            assert_eq!(
                [0x89, 0xf3, 0x01, 0x00],
                transmute::<_, [u8; 4]>(char_ule::from_native('🎉')),
            );
            assert_eq!(
                [0x00, 0x01, 0xf3, 0x89],
                transmute::<_, [u8; 4]>(char_ube::from_native('🎉')),
            );
        }
    }

    #[test]
    fn signed_non_zero() {
        assert_size_align! {
            NonZeroI16_ule 2 1,
            NonZeroI16_ube 2 1,
            NonZeroI32_ule 4 1,
            NonZeroI32_ube 4 1,
            NonZeroI64_ule 8 1,
            NonZeroI64_ube 8 1,
            NonZeroI128_ule 16 1,
            NonZeroI128_ube 16 1,
        }

        unsafe {
            // NonZeroI16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(NonZeroI16_ule::new_unchecked(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(NonZeroI16_ube::new_unchecked(0x0102)),
            );

            // NonZeroI32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(NonZeroI32_ule::new_unchecked(
                    0x01020304
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(NonZeroI32_ube::new_unchecked(
                    0x01020304
                )),
            );

            // NonZeroI64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(NonZeroI64_ule::new_unchecked(
                    0x0102030405060708
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(NonZeroI64_ube::new_unchecked(
                    0x0102030405060708
                )),
            );

            // NonZeroI128
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                    0x06, 0x05, 0x04, 0x03, 0x02, 0x01
                ],
                transmute::<_, [u8; 16]>(NonZeroI128_ule::new_unchecked(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                    0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10
                ],
                transmute::<_, [u8; 16]>(NonZeroI128_ube::new_unchecked(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
        }
    }

    #[test]
    fn unsigned_non_zero() {
        assert_size_align! {
            NonZeroU16_ule 2 1,
            NonZeroU16_ube 2 1,
            NonZeroU32_ule 4 1,
            NonZeroU32_ube 4 1,
            NonZeroU64_ule 8 1,
            NonZeroU64_ube 8 1,
            NonZeroU128_ule 16 1,
            NonZeroU128_ube 16 1,
        }

        unsafe {
            // NonZeroU16
            assert_eq!(
                [0x02, 0x01],
                transmute::<_, [u8; 2]>(NonZeroU16_ule::new_unchecked(0x0102)),
            );
            assert_eq!(
                [0x01, 0x02],
                transmute::<_, [u8; 2]>(NonZeroU16_ube::new_unchecked(0x0102)),
            );

            // NonZeroU32
            assert_eq!(
                [0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 4]>(NonZeroU32_ule::new_unchecked(
                    0x01020304
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04],
                transmute::<_, [u8; 4]>(NonZeroU32_ube::new_unchecked(
                    0x01020304
                )),
            );

            // NonZeroU64
            assert_eq!(
                [0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01],
                transmute::<_, [u8; 8]>(NonZeroU64_ule::new_unchecked(
                    0x0102030405060708
                )),
            );
            assert_eq!(
                [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                transmute::<_, [u8; 8]>(NonZeroU64_ube::new_unchecked(
                    0x0102030405060708
                )),
            );

            // NonZeroU128
            assert_eq!(
                [
                    0x10, 0x0f, 0x0e, 0x0d, 0x0c, 0x0b, 0x0a, 0x09, 0x08, 0x07,
                    0x06, 0x05, 0x04, 0x03, 0x02, 0x01
                ],
                transmute::<_, [u8; 16]>(NonZeroU128_ule::new_unchecked(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
            assert_eq!(
                [
                    0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
                    0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10
                ],
                transmute::<_, [u8; 16]>(NonZeroU128_ube::new_unchecked(
                    0x0102030405060708090a0b0c0d0e0f10
                )),
            );
        }
    }
}
