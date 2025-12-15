use core::cell::Cell;
use munge::munge;

pub union Foo {
    pub a: u32,
    pub b: u8,
}

fn main() {
    let foo = Cell::new(Foo { a: u32::MAX });
    munge!(let Foo { b } = foo);
    b.get();
}
