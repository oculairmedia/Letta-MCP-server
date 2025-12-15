
Initialize `MaybeUninit`s:

```rust
use core::mem::MaybeUninit;
use munge::munge;

pub struct Example {
    a: u32,
    b: (char, f32),
}

let mut mu = MaybeUninit::<Example>::uninit();

munge!(let Example { a, b: (c, mut f) } = &mut mu);
assert_eq!(a.write(10), &10);
assert_eq!(c.write('x'), &'x');
assert_eq!(f.write(3.14), &3.14);
// Note that `mut` bindings can be reassigned like you'd expect:
f = &mut MaybeUninit::uninit();

// SAFETY: `mu` is completely initialized.
let init = unsafe { mu.assume_init() };
assert_eq!(init.a, 10);
assert_eq!(init.b.0, 'x');
assert_eq!(init.b.1, 3.14);
```

Destructure `Cell`s:

```rust
use core::cell::Cell;
use munge::munge;

pub struct Example {
    a: u32,
    b: (char, f32),
}

let value = Example {
    a: 10,
    b: ('x', 3.14),
};
let cell = Cell::<Example>::new(value);

munge!(let Example { a, b: (c, f) } = &cell);
assert_eq!(a.get(), 10);
a.set(42);
assert_eq!(c.get(), 'x');
c.set('!');
assert_eq!(f.get(), 3.14);
f.set(1.41);

let value = cell.into_inner();
assert_eq!(value.a, 42);
assert_eq!(value.b.0, '!');
assert_eq!(value.b.1, 1.41);
```

You can even extend munge to work with your own types by implementing its
`Destructure` and `Restructure` traits:

```rust
use munge::{Destructure, Restructure, Move, munge};

pub struct Invariant<T>(T);

impl<T> Invariant<T> {
    /// # Safety
    ///
    /// `value` must uphold my custom invariant.
    pub unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }

    pub fn unwrap(self) -> T {
        self.0
    }
}

// SAFETY:
// - `Invariant<T>` is destructured by move, so its `Destructuring` type is
//   `Move`.
// - `underlying` returns a pointer to its inner type, so it is guaranteed
//   to be non-null, properly aligned, and valid for reads.
unsafe impl<T> Destructure for Invariant<T> {
    type Underlying = T;
    type Destructuring = Move;

    fn underlying(&mut self) -> *mut Self::Underlying {
        &mut self.0 as *mut Self::Underlying
    }
}

// SAFETY: `restructure` returns an `Invariant<U>` that takes ownership of
// the restructured field because `Invariant<T>` is destructured by move.
unsafe impl<T, U> Restructure<U> for Invariant<T> {
    type Restructured = Invariant<U>;

    unsafe fn restructure(&self, ptr: *mut U) -> Self::Restructured {
        // SAFETY: The caller has guaranteed that `ptr` is a pointer to a
        // subfield of some `T`, so it must be properly aligned, valid for
        // reads, and initialized. We may move the fields because the
        // destructuring type for `Invariant<T>` is `Move`.
        let value = unsafe { ptr.read() };
        Invariant(value)
    }
}

// SAFETY: `(1, 2, 3)` upholds my custom invariant.
let value = unsafe { Invariant::new_unchecked((1, 2, 3)) };
munge!(let (one, two, three) = value);
assert_eq!(one.unwrap(), 1);
assert_eq!(two.unwrap(), 2);
assert_eq!(three.unwrap(), 3);
```
