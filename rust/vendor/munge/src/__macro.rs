use core::{hint::unreachable_unchecked, marker::PhantomData};

use crate::{internal, Borrow, Destructure, Restructure};

pub fn make_destructurer<T: Destructure>(
    value: T,
) -> <T::Destructuring as internal::DestructuringFor<T>>::Destructurer
where
    T::Destructuring: internal::DestructuringFor<T>,
{
    internal::Destructurer::new(value)
}

pub fn destructurer_ptr<T: internal::Destructurer>(
    destructurer: &mut T,
) -> *mut <T::Inner as Destructure>::Underlying {
    Destructure::underlying(destructurer.inner_mut())
}

/// # Safety
///
/// `test_destructurer` may not be called.
pub unsafe fn test_destructurer<'a, T: internal::Test<'a>>(
    _: &'a mut T,
) -> T::Test {
    // SAFETY: `test_destructurer` may not be called.
    unsafe { unreachable_unchecked() }
}

/// # Safety
///
/// `ptr` must be a properly-aligned pointer to a subfield of the pointer
/// underlying the inner value of `destructurer`.
pub unsafe fn restructure_destructurer<T: internal::Destructurer, U>(
    destructurer: &T,
    ptr: *mut U,
) -> <T::Inner as Restructure<U>>::Restructured
where
    T::Inner: Restructure<U>,
{
    // SAFETY: The caller has guaranteed that `ptr` is a properly-aligned
    // pointer to a subfield of the pointer underlying the inner value of
    // `destructurer`.
    unsafe {
        Restructure::restructure(
            internal::Destructurer::inner(destructurer),
            ptr,
        )
    }
}

#[diagnostic::on_unimplemented(
    message = "munge may not destructure a rest pattern by move",
    label = "this rest pattern requires destructuring by borrow"
)]
pub trait MustBeBorrow {}

#[diagnostic::do_not_recommend]
impl MustBeBorrow for Borrow {}

pub fn get_destructuring_ptr<T>(
    _: &T,
) -> *const <T::Inner as Destructure>::Destructuring
where
    T: internal::Destructurer,
    T::Inner: Destructure,
{
    core::ptr::null()
}

pub struct Value;

pub struct Reference;

#[diagnostic::on_unimplemented(
    message = "munge may not destructure through references",
    label = "destructuring with this pattern causes an implicit dereference",
    note = "only values may be destructured"
)]
pub trait MustBeAValue {
    fn check(&self) {}
}

#[diagnostic::do_not_recommend]
impl MustBeAValue for Value {}

pub trait MaybeReference {
    fn test(&self) -> Value {
        Value
    }
}

impl<T: ?Sized> MaybeReference for T {}

pub struct IsReference<T: ?Sized>(PhantomData<T>);

impl<T: ?Sized> IsReference<T> {
    pub fn for_ptr(_: *mut T) -> Self {
        Self(PhantomData)
    }
}

impl<T: ?Sized> IsReference<&mut T> {
    pub fn test(&self) -> Reference {
        Reference
    }
}

impl<T: ?Sized> IsReference<&T> {
    pub fn test(&self) -> Reference {
        Reference
    }
}
