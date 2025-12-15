use core::cell::Cell;
use munge::munge;

fn main() {
    struct Foo<T> {
        value: T,
    }

    impl<T> core::ops::Deref for Foo<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    struct Bar {
        a: Foo<(u32, u32)>,
    }

    let value = Cell::new(Bar { a: Foo { value: (1, 2) }});
    munge!(let Bar { a: (_, _) } = value);
}
