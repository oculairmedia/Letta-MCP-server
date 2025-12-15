use core::mem::ManuallyDrop;

use crate::Destructure;

pub trait Destructuring {}

pub trait DestructuringFor<T>: Destructuring {
    type Destructurer: Destructurer<Inner = T>;
}

pub trait Destructurer {
    type Inner: Destructure;

    fn new(inner: Self::Inner) -> Self;

    fn inner(&self) -> &Self::Inner;

    fn inner_mut(&mut self) -> &mut Self::Inner;
}

pub trait Test<'a> {
    type Test;
}

pub struct Borrow<T>(T);

impl<T: Destructure> Destructurer for Borrow<T> {
    type Inner = T;

    fn new(inner: T) -> Self {
        Self(inner)
    }

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

impl<'a, T: 'a + Destructure> Test<'a> for Borrow<T> {
    type Test = &'a T::Underlying;
}

pub struct Move<T>(ManuallyDrop<T>);

impl<T: Destructure> Destructurer for Move<T> {
    type Inner = T;

    fn new(inner: T) -> Self {
        Self(ManuallyDrop::new(inner))
    }

    fn inner(&self) -> &Self::Inner {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.0
    }
}

impl<'a, T: 'a + Destructure> Test<'a> for Move<T>
where
    T::Underlying: Sized,
{
    type Test = T::Underlying;
}
