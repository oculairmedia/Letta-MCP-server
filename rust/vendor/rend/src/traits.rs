macro_rules! impl_unop {
    ($trait:ident:: $fn:ident for $name:ident : $prim:ty) => {
        impl ::core::ops::$trait for $name {
            type Output = <$prim as ::core::ops::$trait>::Output;

            #[inline]
            fn $fn(self) -> Self::Output {
                self.to_native().$fn()
            }
        }
    };
}

macro_rules! impl_binop_nonzero {
    ($trait:ident::$fn:ident for $name:ident: $prim:ty) => {
        impl_binop_both!($trait::$fn ($name, $prim) -> $prim);
        impl_binop_both!($trait::$fn (&'_ $name, $prim) -> $prim);

        impl_binop_one!($trait::$fn ($name, $name) -> $prim);
        impl_binop_one!($trait::$fn (&'_ $name, $name) -> $prim);
        impl_binop_one!($trait::$fn ($name, &'_ $name) -> $prim);
        impl_binop_one!($trait::$fn (&'_ $name, &'_ $name) -> $prim);
    };
}

macro_rules! impl_binop_one {
    ($trait:ident:: $fn:ident($self:ty, $other:ty) -> $output:ty) => {
        impl ::core::ops::$trait<$other> for $self {
            type Output = $output;

            #[inline]
            fn $fn(self, other: $other) -> Self::Output {
                self.to_native().$fn(other.to_native())
            }
        }
    };
}

macro_rules! impl_binop_both {
    ($trait:ident:: $fn:ident($self:ty, $other:ty) -> $output:ty) => {
        impl ::core::ops::$trait<$other> for $self {
            type Output = $output;

            #[inline]
            fn $fn(self, other: $other) -> Self::Output {
                self.to_native().$fn(other)
            }
        }

        impl ::core::ops::$trait<$self> for $other {
            type Output = $output;

            #[inline]
            fn $fn(self, other: $self) -> Self::Output {
                self.$fn(other.to_native())
            }
        }
    };
}

macro_rules! impl_binop {
    ($trait:ident::$fn:ident for $name:ident: $prim:ty) => {
        impl_binop_both!($trait::$fn ($name, $prim) -> $prim);
        impl_binop_both!($trait::$fn (&'_ $name, $prim) -> $prim);
        impl_binop_both!($trait::$fn ($name, &'_ $prim) -> $prim);
        impl_binop_both!($trait::$fn (&'_ $name, &'_ $prim) -> $prim);

        impl_binop_one!($trait::$fn ($name, $name) -> $prim);
        impl_binop_one!($trait::$fn (&'_ $name, $name) -> $prim);
        impl_binop_one!($trait::$fn ($name, &'_ $name) -> $prim);
        impl_binop_one!($trait::$fn (&'_ $name, &'_ $name) -> $prim);
    };
}

macro_rules! impl_binassign_nonzero {
    ($trait:ident:: $fn:ident for $name:ident : $prim:ty) => {
        impl ::core::ops::$trait<$prim> for $name {
            #[inline]
            fn $fn(&mut self, other: $prim) {
                let mut value = self.to_native();
                value.$fn(other);
                *self = Self::from_native(value);
            }
        }

        impl ::core::ops::$trait<$name> for $name {
            #[inline]
            fn $fn(&mut self, other: $name) {
                let mut value = self.to_native();
                value.$fn(other.to_native());
                *self = Self::from_native(value);
            }
        }
    };
}

macro_rules! impl_binassign {
    ($trait:ident:: $fn:ident for $name:ident : $prim:ty) => {
        impl ::core::ops::$trait<$prim> for $name {
            #[inline]
            fn $fn(&mut self, other: $prim) {
                let mut value = self.to_native();
                value.$fn(other);
                *self = Self::from_native(value);
            }
        }

        impl ::core::ops::$trait<$name> for $name {
            #[inline]
            fn $fn(&mut self, other: $name) {
                let mut value = self.to_native();
                value.$fn(other.to_native());
                *self = Self::from_native(value);
            }
        }

        impl ::core::ops::$trait<&'_ $prim> for $name {
            #[inline]
            fn $fn(&mut self, other: &'_ $prim) {
                let mut value = self.to_native();
                value.$fn(other);
                *self = Self::from_native(value);
            }
        }

        impl ::core::ops::$trait<&'_ $name> for $name {
            #[inline]
            fn $fn(&mut self, other: &'_ $name) {
                let mut value = self.to_native();
                value.$fn(other.to_native());
                *self = Self::from_native(value);
            }
        }
    };
}

macro_rules! impl_clone_and_copy {
    (for $name:ident) => {
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl Copy for $name {}
    };
}

macro_rules! impl_fmt {
    ($trait:ident for $name:ident) => {
        impl ::core::fmt::$trait for $name {
            #[inline]
            fn fmt(
                &self,
                f: &mut ::core::fmt::Formatter<'_>,
            ) -> ::core::fmt::Result {
                ::core::fmt::$trait::fmt(&self.to_native(), f)
            }
        }
    };
}

macro_rules! impl_default {
    (for $name:ident : $prim:ty) => {
        impl Default for $name {
            #[inline]
            fn default() -> Self {
                Self::from_native(<$prim>::default())
            }
        }
    };
}

macro_rules! impl_from {
    (for $name:ident : $prim:ty) => {
        impl From<$prim> for $name {
            fn from(value: $prim) -> Self {
                Self::from_native(value)
            }
        }

        impl<'a> From<&'a $prim> for $name {
            fn from(value: &'a $prim) -> Self {
                Self::from_native(*value)
            }
        }

        impl From<$name> for $prim {
            fn from(value: $name) -> Self {
                value.to_native()
            }
        }

        impl<'a> From<&'a $name> for $prim {
            fn from(value: &'a $name) -> Self {
                value.to_native()
            }
        }
    };
}

macro_rules! impl_try_from_ptr_size {
    ($size:ident for $name:ident: $prim:ident) => {
        impl TryFrom<$size> for $name {
            type Error = <$prim as TryFrom<$size>>::Error;

            #[inline]
            fn try_from(value: $size) -> Result<Self, Self::Error> {
                Ok(Self::from_native(<$prim>::try_from(value)?))
            }
        }

        impl_try_into_ptr_size!($size for $name: $prim);
    };
}

macro_rules! impl_try_into_ptr_size {
    (isize for $name:ident: i16) => {
        impl_into_ptr_size!(isize for $name);
    };

    (usize for $name:ident: u16) => {
        impl_into_ptr_size!(usize for $name);
    };

    ($size:ident for $name:ident: $prim:ident) => {
        impl TryFrom<$name> for $size {
            type Error = <$size as TryFrom<$prim>>::Error;

            #[inline]
            fn try_from(value: $name) -> Result<Self, Self::Error> {
                <$size>::try_from(value.to_native())
            }
        }
    };
}

macro_rules! impl_into_ptr_size {
    ($size:ident for $name:ident) => {
        impl From<$name> for $size {
            #[inline]
            fn from(value: $name) -> Self {
                <$size>::from(value.to_native())
            }
        }
    };
}

macro_rules! impl_hash {
    (for $name:ident) => {
        impl core::hash::Hash for $name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.to_native().hash(state);
            }
        }
    };
}

macro_rules! impl_partial_ord_and_ord {
    (for $name:ident : $prim:ty) => {
        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Self,
            ) -> Option<::core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialOrd<$prim> for $name {
            #[inline]
            fn partial_cmp(
                &self,
                other: &$prim,
            ) -> Option<::core::cmp::Ordering> {
                self.to_native().partial_cmp(other)
            }
        }

        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.to_native().cmp(&other.to_native())
            }
        }
    };
}

macro_rules! impl_partial_eq_and_eq {
    (for $name:ident : $prim:ty) => {
        impl PartialEq for $name {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                let lhs = self.0;
                let rhs = other.0;
                lhs.eq(&rhs)
            }
        }

        impl PartialEq<$prim> for $name {
            #[inline]
            fn eq(&self, other: &$prim) -> bool {
                self.to_native().eq(other)
            }
        }

        impl PartialEq<$name> for $prim {
            #[inline]
            fn eq(&self, other: &$name) -> bool {
                self.eq(&other.to_native())
            }
        }

        impl Eq for $name {}
    };
}

macro_rules! impl_partial_ord {
    (for $name:ident : $prim:ty) => {
        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(
                &self,
                other: &Self,
            ) -> Option<::core::cmp::Ordering> {
                self.to_native().partial_cmp(&other.to_native())
            }
        }

        impl PartialOrd<$prim> for $name {
            #[inline]
            fn partial_cmp(
                &self,
                other: &$prim,
            ) -> Option<::core::cmp::Ordering> {
                self.to_native().partial_cmp(other)
            }
        }
    };
}

macro_rules! impl_product_and_sum {
    (for $name:ident) => {
        impl ::core::iter::Product for $name {
            #[inline]
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self::from_native(iter.map(|x| x.to_native()).product())
            }
        }

        impl ::core::iter::Sum for $name {
            #[inline]
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self::from_native(iter.map(|x| x.to_native()).sum())
            }
        }
    };
}

/// # Safety
///
/// An impl of `CheckBytes` with a `check_bytes` function that is a no-op must
/// be sound for `$name`.
macro_rules! unsafe_impl_check_bytes_noop {
    (for $name:ident) => {
        #[cfg(feature = "bytecheck")]
        // SAFETY: All callers of this macro have guaranteed that all pointers
        // to `$name`s which are properly aligned and point to enough bytes to
        // represent the type also point to a valid instance of the type.
        unsafe impl<C> bytecheck::CheckBytes<C> for $name
        where
            C: bytecheck::rancor::Fallible + ?Sized,
        {
            #[inline]
            unsafe fn check_bytes(
                _: *const Self,
                _: &mut C,
            ) -> Result<(), C::Error> {
                // SAFETY: The invoker of this macro has guaranteed that an impl
                // of `CheckBytes` with a `check_bytes` function that is a no-op
                // is sound.
                Ok(())
            }
        }
    };
}

/// # Safety
///
/// An all-zero bit pattern for `$name` must be a valid value.
/// As a rule, any derivative type (e.g. `u64_le` or `i32_be`) of a native type
/// (e.g. `u64`, `i32`) that is `Zeroable` will also be zeroable.
macro_rules! unsafe_impl_zeroable {
    (for $name:ident) => {
        #[cfg(feature = "bytemuck-1")]
        unsafe impl bytemuck_1::Zeroable for $name {}
    };
}

/// # Safety
///
/// Read the safety requirements of [`bytemuck::Pod`].
/// In general, any type that is natively `Pod` (e.g. `u64`, `AtomicU32`) will
/// be `Pod` even if wrapped (`u64_le`).
///
/// It is required that `$name` has an impl for `Zeroable` for this macro to
/// work. See [`unsafe_impl_zeroable!()`].
macro_rules! unsafe_impl_pod {
    (for $name:ident) => {
        #[cfg(feature = "bytemuck-1")]
        unsafe impl bytemuck_1::Pod for $name {}
    };
}

/// # Safety
///
/// Read the safety requirements of [`bytemuck::NoUninit`].
/// All primitive types are `NoUninit` and as a result, all of this crates's
/// wrapped primitive types are also `NoUninit`.
macro_rules! unsafe_impl_no_uninit {
    (for $name:ident) => {
        #[cfg(feature = "bytemuck-1")]
        unsafe impl bytemuck_1::NoUninit for $name {}
    };
}
