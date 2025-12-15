//! A radioactive stabilization of the [`ptr_meta` RFC][rfc].
//!
//! This crate provides the [`Pointee`] trait, [`from_raw_parts`] and
//! [`to_raw_parts`] functions, and proc macros for deriving `Pointee` for
//! structs and implementing `Pointee` for trait objects.
//!
//! [rfc]: https://rust-lang.github.io/rfcs/2580-ptr-meta.html
//!
//! # Usage
//!
//! Raw pointers can be decomposed into the data address and metadata components
//! with [`to_raw_parts`] or [`to_raw_parts_mut`].
//!
//! Alternatively, metadata alone can be extracted with the [`metadata`]
//! function. Although [`metadata`] accepts pointers, references can be passed
//! and will be implicitly coerced.
//!
//! A pointer can be created from its address and metadata with
//! [`from_raw_parts`] or [`from_raw_parts_mut`].
//!
//! ## Provided impls
//!
//! `ptr_meta` provides inherent implementations for many builtin types:
//!
//! - All [`Sized`] types implement [`Pointee`] via a blanket implementation.
//! - `slice`, `str`, and `CStr`
//! - `OsStr` (requires `std`)
//! - `dyn Any`, optionally with `+ Send` and/or `+ Sync`
//! - `dyn Error`, optionally with `+ Send` and/or `+ Sync`
//!
//! ## Structs with trailing DSTs
//!
//! You can derive [`Pointee`] for structs with trailing DSTs:
//!
//! ```
//! use ptr_meta::Pointee;
//!
//! #[derive(Pointee)]
//! struct Block<H, T> {
//!     header: H,
//!     elements: [T],
//! }
//! ```
//!
//! Note that the last field is required to be a DST. Structs with a generic
//! type as the last field may have conflicting blanket implementations, as the
//! generic type may be `Sized`. A collection of specific implementations may be
//! required in these cases, with the generic parameter set (for example) a
//! slice, `str`, or specific trait object.
//!
//! ## Trait objects
//!
//! You can generate [`Pointee`] implementations for trait objects:
//!
//! ```
//! use ptr_meta::pointee;
//!
//! // Generates Pointee for dyn Stringy
//! #[ptr_meta::pointee]
//! trait Stringy {
//!     fn as_string(&self) -> String;
//! }
//! ```
//!
//! Note that this will not produce implementations for `Trait + Send + Sync`.
//!
//! ## Features
//!
//! - `derive`: Re-exports the macros from `ptr_meta_derive`. Enabled by
//!   default.
//! - `std`: Enables additional impls for `std` types. Enabled by default.
//!
//! ## Example
#![doc = include_str!("../example.md")]
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
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(docsrs, not(doctest)), feature(doc_cfg))]
#![cfg_attr(miri, allow(internal_features), feature(core_intrinsics))]

mod impls;

use core::{
    ffi::CStr,
    fmt,
    hash::{Hash, Hasher},
};

#[cfg(feature = "derive")]
pub use ptr_meta_derive::{pointee, Pointee};

/// A trait which associates pointer metadata with a pointee type.
///
/// # Pointer metadata
///
/// Pointers and references can be thought of as having two parts: a data
/// address and some extra "pointer metadata".
///
/// Pointers to [statically-sized types](`Sized`) and `extern` types are
/// "narrow": their pointer metadata is `()`.
///
/// Pointers to [dynamically-sized types][dst] are "wide": they have pointer
/// metadata with a non-zero size. There are four classes of dynamically-sized
/// types currently available:
///
/// * `str`s have `usize` pointer metadata equal to the length of the string
///   slice in bytes.
/// * Slices like `[i32]` have `usize` pointer metadata equal to the length of
///   the slice in items.
/// * Trait objects like `dyn SomeTrait` have [`DynMetadata`] pointer metadata,
///   which point to the trait objects' virtual method tables.
/// * Structs with a trailing DST have the same metadata as the trailing DST.
///
/// In the future, Rust may add new kinds of types which have different pointer
/// metadata.
///
/// [dst]: https://doc.rust-lang.org/reference/dynamically-sized-types.html
///
/// # Safety
///
/// The associated `Metadata` type must be the pointer metadata type for the
/// implementing type.
pub unsafe trait Pointee {
    /// The metadata type for pointers and references to this type.
    type Metadata: Copy + Send + Sync + Ord + Hash + Unpin;
}

// SAFETY: Pointers to `Sized` types have no metadata (i.e. their metadata is
// the unit type `()`).
unsafe impl<T> Pointee for T {
    type Metadata = ();
}

// SAFETY: Pointers to slices have a `usize` representing the length of the
// slice in elements as their metadata.
unsafe impl<T> Pointee for [T] {
    type Metadata = usize;
}

// SAFETY: String slice pointers have a `usize` representing the length of the
// string slice in bytes as their metadata.
unsafe impl Pointee for str {
    type Metadata = usize;
}

// SAFETY: `CStr` pointers have a `usize` representing the length of the
// C-string slice in bytes (nul included) as their metadata.
unsafe impl Pointee for CStr {
    type Metadata = usize;
}

#[cfg(feature = "std")]
// SAFETY: `OsStr` pointers have a `usize` representing the length of the
// string in bytes as their metadata.
unsafe impl Pointee for std::ffi::OsStr {
    type Metadata = usize;
}

/// Returns the metadata of the given pointer.
///
/// `*mut T`, `&T`, and `&mut T` can all be passed directly to this function as
/// they implicitly coerce to `*const T`.
///
/// # Example
///
/// ```
/// // String slices have pointer metadata equal to their size in bytes
/// assert_eq!(ptr_meta::metadata("foo"), 3_usize);
/// ```
#[inline]
pub const fn metadata<T: Pointee + ?Sized>(
    ptr: *const T,
) -> <T as Pointee>::Metadata {
    // SAFETY: Accessing the value from the `PtrRepr` union is safe since
    // *const T and PtrComponents<T> have the same memory layouts. Only std can
    // make this guarantee.
    unsafe { PtrRepr { const_ptr: ptr }.components.metadata }
}

/// Returns the data address and metadata of the given pointer.
///
/// `*mut T`, `&T`, and `&mut T` can all be passed directly to this function as
/// they implicitly coerce to `*const T`.
///
/// # Example
///
/// ```
/// let (data_address, metadata) = ptr_meta::to_raw_parts("foo");
/// assert_ne!(data_address, core::ptr::null());
/// assert_eq!(metadata, 3);
/// ```
#[inline]
pub const fn to_raw_parts<T: Pointee + ?Sized>(
    ptr: *const T,
) -> (*const (), <T as Pointee>::Metadata) {
    (ptr as *const (), metadata(ptr))
}

/// Returns the mutable data address and metadata of the given pointer.
///
/// See [`to_raw_parts`] for more details.
#[inline]
pub const fn to_raw_parts_mut<T: Pointee + ?Sized>(
    ptr: *mut T,
) -> (*mut (), <T as Pointee>::Metadata) {
    (ptr as *mut (), metadata(ptr))
}

/// Returns a raw pointer with the given data address and metadata.
///
/// This function is safe, but the returned pointer is not necessarily safe to
/// dereference. For slices, see the documentation of [`slice::from_raw_parts`]
/// for safety requirements. For trait objects, the metadata must come from a
/// a trait object with the same underlying type.
///
/// [`slice::from_raw_parts`]: core::slice::from_raw_parts
#[inline]
pub const fn from_raw_parts<T: Pointee + ?Sized>(
    data_address: *const (),
    metadata: <T as Pointee>::Metadata,
) -> *const T {
    // SAFETY: Accessing the value from the `PtrRepr` union is safe since
    // *const T and PtrComponents<T> have the same memory layouts. Only std can
    // make this guarantee.
    unsafe {
        PtrRepr {
            components: PtrComponents {
                data_address,
                metadata,
            },
        }
        .const_ptr
    }
}

/// Returns a mutable raw pointer with the given data address and metadata.
///
/// See [`from_raw_parts`] for more details.
#[inline]
pub const fn from_raw_parts_mut<T: Pointee + ?Sized>(
    data_address: *mut (),
    metadata: <T as Pointee>::Metadata,
) -> *mut T {
    // SAFETY: Accessing the value from the `PtrRepr` union is safe since
    // *const T and PtrComponents<T> have the same memory layouts. Only std can
    // make this guarantee.
    unsafe {
        PtrRepr {
            components: PtrComponents {
                data_address,
                metadata,
            },
        }
        .mut_ptr
    }
}

#[repr(C)]
union PtrRepr<T: Pointee + ?Sized> {
    const_ptr: *const T,
    mut_ptr: *mut T,
    components: PtrComponents<T>,
}

#[repr(C)]
struct PtrComponents<T: Pointee + ?Sized> {
    data_address: *const (),
    metadata: <T as Pointee>::Metadata,
}

// Manual impl needed to avoid `T: Copy` bound.
impl<T: Pointee + ?Sized> Copy for PtrComponents<T> {}

// Manual impl needed to avoid `T: Clone` bound.
impl<T: Pointee + ?Sized> Clone for PtrComponents<T> {
    fn clone(&self) -> Self {
        *self
    }
}

/// The metadata for a trait object.
///
/// This struct wraps a pointer to a vtable (virtual method table) which
/// contains all of the necessary information to manipulate the concrete type
/// stored inside of the trait object:
///
/// * The size and alignment of the concrete type
/// * A function pointer to the type's `drop_in_place` impl
/// * Function pointers for each method in the concrete type's trait
///   implementation
///
/// Providing a type argument that is not a `dyn` trait object is possible, but
/// does not correspond with a meaningful type.
pub struct DynMetadata<Dyn: ?Sized> {
    vtable_ptr: &'static VTable,
    phantom: core::marker::PhantomData<Dyn>,
}

// Extern types are not stable, so we substitute a ZST. This is not a perfect
// substitute but it's not exposed anywhere so it's close enough.
struct VTable;

impl<Dyn: ?Sized> DynMetadata<Dyn> {
    /// Returns the size of the type associated with this metadata.
    #[inline]
    pub fn size_of(self) -> usize {
        #[cfg(miri)]
        {
            // Note that "size stored in vtable" is *not* the same as "result of
            // size_of_val_raw". Consider a reference like `&(i32,
            // dyn Send)`: the vtable will only store the size of the
            // `Send` part!
            // SAFETY: DynMetadata always contains a valid vtable pointer
            return unsafe {
                core::intrinsics::vtable_size(
                    self.vtable_ptr as *const VTable as *const (),
                )
            };
        }
        #[cfg(not(miri))]
        {
            // SAFETY: This happens to be true. It may not always be true. The
            // location of the size for vtables is based on the implementation
            // of the vtable_size intrinsic.
            unsafe {
                (self.vtable_ptr as *const VTable as *const usize)
                    .add(1)
                    .read()
            }
        }
    }

    /// Returns the alignment of the type associated with this metadata.
    #[inline]
    pub fn align_of(self) -> usize {
        #[cfg(miri)]
        {
            // SAFETY: DynMetadata always contains a valid vtable pointer
            return unsafe {
                core::intrinsics::vtable_align(
                    self.vtable_ptr as *const VTable as *const (),
                )
            };
        }
        #[cfg(not(miri))]
        {
            // SAFETY: This happens to be true. It may not always be true. The
            // location of the alignment for vtables is based on the
            // implementation of the vtable_align intrinsic.
            unsafe {
                (self.vtable_ptr as *const VTable as *const usize)
                    .add(2)
                    .read()
            }
        }
    }

    /// Returns the layout of the type associated with this metadata.
    #[inline]
    pub fn layout(self) -> core::alloc::Layout {
        // SAFETY: the compiler emitted this vtable for a concrete Rust type
        // which is known to have a valid layout. Same rationale as in
        // `Layout::for_value`.
        unsafe {
            core::alloc::Layout::from_size_align_unchecked(
                self.size_of(),
                self.align_of(),
            )
        }
    }
}

// SAFETY: References to trait object vtables are guaranteed to be `Send`.
unsafe impl<Dyn: ?Sized> Send for DynMetadata<Dyn> {}
// SAFETY: References to trait object vtables are guaranteed to be `Sync`.
unsafe impl<Dyn: ?Sized> Sync for DynMetadata<Dyn> {}

impl<Dyn: ?Sized> fmt::Debug for DynMetadata<Dyn> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DynMetadata")
            .field(&(self.vtable_ptr as *const VTable))
            .finish()
    }
}

// Manual impls needed to avoid `Dyn: $Trait` bounds.

impl<Dyn: ?Sized> Unpin for DynMetadata<Dyn> {}

impl<Dyn: ?Sized> Copy for DynMetadata<Dyn> {}

impl<Dyn: ?Sized> Clone for DynMetadata<Dyn> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<Dyn: ?Sized> Eq for DynMetadata<Dyn> {}

impl<Dyn: ?Sized> PartialEq for DynMetadata<Dyn> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        core::ptr::eq::<VTable>(self.vtable_ptr, other.vtable_ptr)
    }
}

impl<Dyn: ?Sized> Ord for DynMetadata<Dyn> {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        (self.vtable_ptr as *const VTable)
            .cmp(&(other.vtable_ptr as *const VTable))
    }
}

impl<Dyn: ?Sized> PartialOrd for DynMetadata<Dyn> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Dyn: ?Sized> Hash for DynMetadata<Dyn> {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        core::ptr::hash::<VTable, _>(self.vtable_ptr, hasher)
    }
}

#[cfg(test)]
fn test_pointee<T: Pointee + ?Sized>(value: &T) {
    let ptr = value as *const T;
    let (raw, meta) = to_raw_parts(ptr);
    let re_ptr = from_raw_parts::<T>(raw, meta);
    assert_eq!(ptr, re_ptr);
}

#[cfg(test)]
mod tests {
    use super::test_pointee;

    #[test]
    fn sized_types() {
        test_pointee(&());
        test_pointee(&42);
        test_pointee(&true);
        test_pointee(&[1, 2, 3, 4]);

        struct TestUnit;

        test_pointee(&TestUnit);

        #[allow(dead_code)]
        struct TestStruct {
            a: (),
            b: i32,
            c: bool,
        }

        test_pointee(&TestStruct {
            a: (),
            b: 42,
            c: true,
        });

        #[allow(dead_code)]
        struct TestTuple((), i32, bool);

        test_pointee(&TestTuple((), 42, true));

        struct TestGeneric<T>(T);

        test_pointee(&TestGeneric(42));
    }

    #[test]
    fn unsized_types() {
        test_pointee("hello world");
        test_pointee(&[1, 2, 3, 4] as &[i32]);
    }
}

#[cfg(all(test, feature = "derive"))]
mod derive_tests {
    use core::any::Any;

    use super::{test_pointee, Pointee};

    #[test]
    fn trait_objects() {
        #[crate::pointee(crate)]
        trait TestTrait {
            #[allow(dead_code)]
            fn foo(&self);
        }

        struct A;

        impl TestTrait for A {
            fn foo(&self) {}
        }

        let trait_object = &A as &dyn TestTrait;

        test_pointee(trait_object);

        #[allow(dead_code)]
        struct B(i32);

        impl TestTrait for B {
            fn foo(&self) {}
        }

        let b = B(42);
        let trait_object = &b as &dyn TestTrait;

        test_pointee(trait_object);
    }

    #[test]
    fn last_field_dst() {
        #[allow(dead_code)]
        #[derive(Pointee)]
        #[ptr_meta(crate)]
        struct Test<H, T> {
            head: H,
            tail: [T],
        }

        #[allow(dead_code)]
        #[derive(Pointee)]
        #[ptr_meta(crate)]
        struct TestDyn {
            tail: dyn Any,
        }

        #[crate::pointee(crate)]
        trait TestTrait {}

        #[allow(dead_code)]
        #[derive(Pointee)]
        #[ptr_meta(crate)]
        struct TestCustomDyn {
            tail: dyn TestTrait,
        }
    }

    #[test]
    fn generic_trait() {
        #[allow(dead_code)]
        #[crate::pointee(crate)]
        trait TestTrait<T: ?Sized> {}

        impl<T: ?Sized> TestTrait<T> for () {}

        test_pointee(&() as &dyn TestTrait<u32>);
    }
}
