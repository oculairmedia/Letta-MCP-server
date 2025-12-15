//! Munge makes it easy and safe to destructure `MaybeUninit`s, `Cell`s,
//! `UnsafeCell`s, `ManuallyDrop`s and more.
//!
//! Just use the `munge!` macro to destructure opaque types the same way you'd
//! destructure a value. The `munge!` macro may be used to perform either borrow
//! destructuring (e.g. `let (a, b) = c` where `c` is a reference) or move
//! destructuring (e.g. `let (a, b) = c` where `c` is a value) depending on the
//! type.
//!
//! Munge has no features and is always `#![no_std]`.
//!
//! ## Examples
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

#[doc(hidden)]
pub mod __macro;
mod impls;
mod internal;

#[doc(hidden)]
pub use munge_macro::munge_with_path;

/// Destructures a type using a pattern.
///
/// To prevent unsound union destructurings, this macro emits field accesses
/// which fail to compile in safe Rust. However, if `munge!` is used inside of
/// an `unsafe` block, these accesses will compile without emitting an error.
/// This matches the behavior of regular destructuring, but may be surprising in
/// some situations.
///
/// # Example
///
/// ```
/// # use core::mem::MaybeUninit;
/// # use munge::munge;
/// pub struct Example {
///     a: u32,
///     b: (char, f32),
/// }
///
/// let mut mu = MaybeUninit::<Example>::uninit();
///
/// munge!(let Example { a, b: (c, mut f) } = &mut mu);
/// assert_eq!(a.write(10), &10);
/// assert_eq!(c.write('x'), &'x');
/// assert_eq!(f.write(3.14), &3.14);
/// // Note that `mut` bindings can be reassigned like you'd expect:
/// let mut new_f = MaybeUninit::uninit();
/// f = &mut new_f;
///
/// // SAFETY: `mu` is completely initialized.
/// let init = unsafe { mu.assume_init() };
/// assert_eq!(init.a, 10);
/// assert_eq!(init.b.0, 'x');
/// assert_eq!(init.b.1, 3.14);
/// ```
#[macro_export]
macro_rules! munge {
    ($($t:tt)*) => { $crate::munge_with_path!($crate => $($t)*) }
}

/// A type that can be destructured into its constituent parts.
///
/// See the [crate docs](index.html#examples) for an example of implementing
/// `Destructure` and `Restructure`.
///
/// # Safety
///
/// - [`Destructuring`](Destructure::Destructuring) must reflect the type of
///   destructuring allowed for the type:
///   - [`Borrow`] if the type is restructured by creating disjoint borrows of
///     the fields of `Underlying`.
///   - [`Move`] if the type may be restructured by moving the fields out of the
///     destructured `Underlying`.
/// - [`underlying`](Destructure::underlying) must return a pointer that is
///   non-null, properly aligned, and valid for reads.
pub unsafe trait Destructure: Sized {
    /// The underlying type that is destructured.
    type Underlying: ?Sized;
    /// The type of destructuring to perform.
    type Destructuring: internal::Destructuring;

    /// Returns a mutable pointer to the underlying type.
    fn underlying(&mut self) -> *mut Self::Underlying;
}

/// A type that can be "restructured" as a field of some containing type.
///
/// See the [crate docs](index.html#examples) for an example of implementing
/// `Destructure` and `Restructure`.
///
/// # Safety
///
/// [`restructure`](Restructure::restructure) must return a valid
/// [`Restructured`](Restructure::Restructured) that upholds the invariants for
/// its [`Destructuring`](Destructure::Destructuring):
/// - If the type is destructured [by borrow](Borrow), then the `Restructured`
///   value must behave as a disjoint borrow of a field of the underlying type.
/// - If the type is destructured [by move](Move), then the `Restructured` value
///   must move the fields out of the underlying type.
pub unsafe trait Restructure<T: ?Sized>: Destructure {
    /// The restructured version of this type.
    type Restructured;

    /// Restructures a pointer to this type into the target type.
    ///
    /// # Safety
    ///
    /// `ptr` must be a properly aligned pointer to a subfield of the pointer
    /// [`underlying`](Destructure::underlying) `self`.
    unsafe fn restructure(&self, ptr: *mut T) -> Self::Restructured;
}

/// Destructuring by borrow, e.g. `let (a, b) = c` where `c` is a reference.
///
/// Borrow destructuring leaves the original value intact, only borrowing from
/// the destructured value. Borrow destructuring may use rest patterns (`..`)
/// because the original value is not moved and so it is safe to restructure
/// only some of the fields of the destructured value.
pub struct Borrow;

impl internal::Destructuring for Borrow {}

impl<T: Destructure> internal::DestructuringFor<T> for Borrow {
    type Destructurer = internal::Borrow<T>;
}

/// Destructuring by move, e.g. `let (a, b) = c` where `c` is a value.
///
/// Move destructuring forgets the original value and moves each destructured
/// field during restructuring. Move destructuring may not use rest patterns
/// (`..`) because every field of the original value must be restructured, else
/// they will be forgotten.
pub struct Move;

impl internal::Destructuring for Move {}

impl<T: Destructure> internal::DestructuringFor<T> for Move {
    type Destructurer = internal::Move<T>;
}

#[cfg(test)]
mod tests {
    use core::{cell::Cell, mem::MaybeUninit};

    #[test]
    fn project_tuple() {
        let mut mu = MaybeUninit::<(u32, char)>::uninit();

        munge!(let (a, b) = &mut mu);
        assert_eq!(a.write(1), &1);
        assert_eq!(b.write('a'), &'a');
        munge!(let (a, b,) = &mut mu);
        assert_eq!(a.write(2), &2);
        assert_eq!(b.write('b'), &'b');

        munge!(let (a, _) = &mut mu);
        assert_eq!(a.write(3), &3);
        munge!(let (_, b) = &mut mu);
        assert_eq!(b.write('c'), &'c');
        munge!(let (a, _,) = &mut mu);
        assert_eq!(a.write(3), &3);
        munge!(let (_, b,) = &mut mu);
        assert_eq!(b.write('c'), &'c');

        munge!(let (mut a, mut b) = &mut mu);
        assert_eq!(a.write(4), &4);
        assert_eq!(b.write('d'), &'d');
        let mut new_a = MaybeUninit::uninit();
        a = &mut new_a;
        let mut new_b = MaybeUninit::uninit();
        b = &mut new_b;
        let _ = a;
        let _ = b;

        munge!(let (a, ..) = &mut mu);
        assert_eq!(a.write(5), &5);

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init.0, 5);
        assert_eq!(init.1, 'd');
    }

    #[test]
    fn project_array() {
        let mut mu = MaybeUninit::<[u32; 2]>::uninit();

        munge!(let [a, b] = &mut mu);
        assert_eq!(a.write(1), &1);
        assert_eq!(b.write(1), &1);
        munge!(let [a, b,] = &mut mu);
        assert_eq!(a.write(2), &2);
        assert_eq!(b.write(2), &2);

        munge!(let [a, _] = &mut mu);
        assert_eq!(a.write(3), &3);
        munge!(let [_, b] = &mut mu);
        assert_eq!(b.write(3), &3);
        munge!(let [a, _,] = &mut mu);
        assert_eq!(a.write(4), &4);
        munge!(let [_, b,] = &mut mu);
        assert_eq!(b.write(4), &4);

        munge!(let [mut a, mut b] = &mut mu);
        assert_eq!(a.write(5), &5);
        assert_eq!(b.write(5), &5);
        let mut new_a = MaybeUninit::uninit();
        a = &mut new_a;
        let mut new_b = MaybeUninit::uninit();
        b = &mut new_b;
        let _ = a;
        let _ = b;

        munge!(let [a, ..] = &mut mu);
        assert_eq!(a.write(6), &6);

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init[0], 6);
        assert_eq!(init[1], 5);
    }

    #[test]
    fn project_struct() {
        pub struct Example {
            pub a: u32,
            pub b: char,
        }

        let mut mu = MaybeUninit::<Example>::uninit();

        munge!(let Example { a, b } = &mut mu);
        assert_eq!(a.write(1), &1);
        assert_eq!(b.write('a'), &'a');
        munge!(let Example { a, b, } = &mut mu);
        assert_eq!(a.write(2), &2);
        assert_eq!(b.write('b'), &'b');

        munge!(let Example { a, b: x } = &mut mu);
        assert_eq!(a.write(3), &3);
        assert_eq!(x.write('c'), &'c');
        munge!(let Example { a, b: x, } = &mut mu);
        assert_eq!(a.write(4), &4);
        assert_eq!(x.write('d'), &'d');

        munge!(let Example { a: x, b } = &mut mu);
        assert_eq!(x.write(3), &3);
        assert_eq!(b.write('c'), &'c');
        munge!(let Example { a: x, b, } = &mut mu);
        assert_eq!(x.write(4), &4);
        assert_eq!(b.write('d'), &'d');

        munge!(let Example { a, b: _ } = &mut mu);
        assert_eq!(a.write(5), &5);
        munge!(let Example { a, b: _, } = &mut mu);
        assert_eq!(a.write(6), &6);

        munge!(let Example { mut a, mut b } = &mut mu);
        assert_eq!(a.write(7), &7);
        assert_eq!(b.write('e'), &'e');
        let mut new_a = MaybeUninit::uninit();
        a = &mut new_a;
        let mut new_b = MaybeUninit::uninit();
        b = &mut new_b;
        let _ = a;
        let _ = b;

        munge!(let Example { a: mut x, b: mut y } = &mut mu);
        assert_eq!(x.write(8), &8);
        assert_eq!(y.write('f'), &'f');
        let mut new_x = MaybeUninit::uninit();
        x = &mut new_x;
        let mut new_y = MaybeUninit::uninit();
        y = &mut new_y;
        let _ = x;
        let _ = y;

        munge!(let Example { b, .. } = &mut mu);
        assert_eq!(b.write('g'), &'g');

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init.a, 8);
        assert_eq!(init.b, 'g');
    }

    #[test]
    fn project_tuple_struct() {
        struct Example(u32, char);

        let mut mu = MaybeUninit::<Example>::uninit();

        munge!(let Example(a, b) = &mut mu);
        assert_eq!(a.write(1), &1);
        assert_eq!(b.write('a'), &'a');
        munge!(let Example(a, b,) = &mut mu);
        assert_eq!(a.write(2), &2);
        assert_eq!(b.write('b'), &'b');

        munge!(let Example(a, _) = &mut mu);
        assert_eq!(a.write(3), &3);
        munge!(let Example(_, b) = &mut mu);
        assert_eq!(b.write('c'), &'c');
        munge!(let Example(a, _,) = &mut mu);
        assert_eq!(a.write(3), &3);
        munge!(let Example(_, b,) = &mut mu);
        assert_eq!(b.write('d'), &'d');

        munge!(let Example(mut a, mut b) = &mut mu);
        assert_eq!(a.write(4), &4);
        assert_eq!(b.write('e'), &'e');
        let mut new_a = MaybeUninit::uninit();
        a = &mut new_a;
        let mut new_b = MaybeUninit::uninit();
        b = &mut new_b;
        let _ = a;
        let _ = b;

        munge!(let Example(a, ..) = &mut mu);
        assert_eq!(a.write(5), &5);

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init.0, 5);
        assert_eq!(init.1, 'e');
    }

    #[test]
    fn project_generic() {
        struct Example<T>(u32, T);

        let mut mu = MaybeUninit::<Example<char>>::uninit();

        munge!(let Example(a, b) = &mut mu);
        assert_eq!(a.write(1), &1);
        assert_eq!(b.write('a'), &'a');
        munge!(let Example(a, b,) = &mut mu);
        assert_eq!(a.write(2), &2);
        assert_eq!(b.write('b'), &'b');

        munge!(let Example(a, _) = &mut mu);
        assert_eq!(a.write(3), &3);
        munge!(let Example(_, b) = &mut mu);
        assert_eq!(b.write('c'), &'c');
        munge!(let Example(a, _,) = &mut mu);
        assert_eq!(a.write(3), &3);
        munge!(let Example(_, b,) = &mut mu);
        assert_eq!(b.write('c'), &'c');

        munge!(let Example(a, ..) = &mut mu);
        assert_eq!(a.write(4), &4);

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init.0, 4);
        assert_eq!(init.1, 'c');

        let mut mu = MaybeUninit::<Example<Example<char>>>::uninit();

        munge!(
            let Example::<Example<char>>(a, Example::<char>(b, c)) = &mut mu;
        );
        assert_eq!(a.write(1), &1);
        assert_eq!(b.write(2), &2);
        assert_eq!(c.write('a'), &'a');

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init.0, 1);
        assert_eq!(init.1 .0, 2);
        assert_eq!(init.1 .1, 'a');
    }

    #[test]
    fn project_nested_struct() {
        struct Inner {
            a: u32,
            b: char,
        }
        struct Outer {
            inner: Inner,
            c: i32,
        }

        let mut mu = MaybeUninit::<Outer>::uninit();

        munge!(let Outer { inner: Inner { a, b }, c } = &mut mu);
        assert_eq!(a.write(1), &1);
        assert_eq!(b.write('a'), &'a');
        assert_eq!(c.write(2), &2);

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init.inner.a, 1);
        assert_eq!(init.inner.b, 'a');
        assert_eq!(init.c, 2);
    }

    #[test]
    fn project_nested_tuple() {
        let mut mu = MaybeUninit::<(u32, (char, u32))>::uninit();

        munge!(let (a, (b, c)) = &mut mu);
        assert_eq!(a.write(1), &1);
        assert_eq!(b.write('a'), &'a');
        assert_eq!(c.write(2), &2);

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init, (1, ('a', 2)));
    }

    #[test]
    fn project_nested_array() {
        let mut mu = MaybeUninit::<[[u32; 2]; 2]>::uninit();

        munge!(let [a, [b, c]] = &mut mu);
        assert_eq!(a.write([1, 2]), &[1, 2]);
        assert_eq!(b.write(3), &3);
        assert_eq!(c.write(4), &4);

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init, [[1, 2], [3, 4]]);
    }

    #[test]
    fn generics() {
        struct Inner<T> {
            a: u32,
            b: T,
        }
        struct Outer<T> {
            inner: Inner<T>,
            c: i32,
        }

        let mut mu = MaybeUninit::<Outer<char>>::uninit();

        munge!(let Outer { inner: Inner { a, b }, c } = &mut mu);
        assert_eq!(a.write(1), &1);
        assert_eq!(b.write('a'), &'a');
        assert_eq!(c.write(2), &2);

        // SAFETY: `mu` is completely initialized.
        let init = unsafe { mu.assume_init() };
        assert_eq!(init.inner.a, 1);
        assert_eq!(init.inner.b, 'a');
        assert_eq!(init.c, 2);
    }

    #[test]
    fn cell() {
        use core::cell::Cell;

        pub struct Example {
            a: u32,
            b: (char, f32),
        }

        let value = Example {
            a: 10,
            b: ('x', core::f32::consts::PI),
        };
        let cell = Cell::<Example>::new(value);

        munge!(let Example { a, b: (c, f) } = &cell);
        assert_eq!(a.get(), 10);
        a.set(42);
        assert_eq!(c.get(), 'x');
        c.set('!');
        assert_eq!(f.get(), core::f32::consts::PI);
        f.set(1.41);

        let value = cell.into_inner();
        assert_eq!(value.a, 42);
        assert_eq!(value.b.0, '!');
        assert_eq!(value.b.1, 1.41);
    }

    #[test]
    fn maybe_uninit_value() {
        let mu = MaybeUninit::<(u32, char)>::new((10_000, 'x'));

        munge!(let (a, b) = mu);
        assert_eq!(unsafe { a.assume_init() }, 10_000);
        assert_eq!(unsafe { b.assume_init() }, 'x');
    }

    #[test]
    fn cell_value() {
        use core::cell::Cell;

        let cell = Cell::<(u32, char)>::new((10_000, 'x'));

        munge!(let (a, b) = cell);
        assert_eq!(a.get(), 10_000);
        assert_eq!(b.get(), 'x');
    }

    #[test]
    fn unsafe_cell_value() {
        use core::cell::UnsafeCell;

        let uc = UnsafeCell::<(u32, char)>::new((10_000, 'x'));

        munge!(let (mut a, mut b) = uc);
        assert_eq!(*a.get_mut(), 10_000);
        assert_eq!(*b.get_mut(), 'x');
    }

    #[test]
    fn manually_drop_value() {
        use core::mem::ManuallyDrop;

        let md = ManuallyDrop::new((10_000, 'x'));

        munge!(let (a, b) = md);
        assert_eq!(*a, 10_000);
        assert_eq!(*b, 'x');
    }

    #[test]
    fn struct_borrow_partial_destructuring() {
        use core::cell::Cell;

        struct Example {
            a: u32,
            b: u32,
        }

        let mut value = Cell::new(Example { a: 0, b: 1 });

        munge!(let Example { a, .. } = &mut value);
        assert_eq!(a.get(), 0);
        a.set(2);
        assert_eq!(a.get(), 2);

        munge!(let Example { a: c, b: _ } = &value);
        assert_eq!(c.get(), 2);
        c.set(3);
        assert_eq!(c.get(), 3);
    }

    #[test]
    fn tuple_borrow_partial_destructuring() {
        use core::cell::Cell;

        struct Example(u32, u32);

        let mut value = Cell::new(Example(0, 1));

        munge!(let Example(a, ..) = &mut value);
        assert_eq!(a.get(), 0);
        a.set(2);
        assert_eq!(a.get(), 2);

        munge!(let Example(c, _) = &value);
        assert_eq!(c.get(), 2);
        c.set(3);
        assert_eq!(c.get(), 3);
    }

    struct NoisyDrop<'a> {
        flag: &'a mut bool,
    }

    impl Drop for NoisyDrop<'_> {
        fn drop(&mut self) {
            *self.flag = true;
        }
    }

    #[test]
    fn moved_tuple_wildcard_pattern_is_dropped() {
        let mut flag = false;
        let value = Cell::new((NoisyDrop { flag: &mut flag },));

        munge!(let (_,) = value);

        assert!(flag);
    }

    #[test]
    fn moved_tuple_struct_wildcard_pattern_is_dropped() {
        struct Example<'a>(NoisyDrop<'a>);

        let mut flag = false;
        let value = Cell::new(Example(NoisyDrop { flag: &mut flag }));

        munge!(let Example(_) = value);

        assert!(flag);
    }

    #[test]
    fn moved_array_wildcard_pattern_is_dropped() {
        let mut flag = false;
        let value = Cell::new([NoisyDrop { flag: &mut flag }]);

        munge!(let [_] = value);

        assert!(flag);
    }

    #[test]
    fn rest_in_full_tuple_pattern() {
        let (_, _, ..) = (1, 2);

        let value = Cell::new((1, 2));
        munge!(let (_, _, ..) = &value);

        let [_, _, ..] = [1, 2];

        let value = Cell::new([1, 2]);
        munge!(let [_, _, ..] = &value);

        struct Foo(i32, i32);

        let Foo(_, _, ..) = Foo(1, 2);

        let value = Cell::new(Foo(1, 2));
        munge!(let Foo(_, _, ..) = &value);

        struct Bar {
            a: i32,
            b: i32,
        }

        let Bar { a: _, b: _, .. } = Bar { a: 1, b: 2 };

        let value = Cell::new(Bar { a: 1, b: 2 });
        munge!(let Bar { a: _, b: _, .. } = &value);
    }
}
