use core::{cell::Cell, mem::MaybeUninit};
use munge::munge;

struct Foo<'a>(&'a mut (i32, i32));

fn main() {
    let mut tuple = (1, 2);
    let mut value = Cell::new(Foo(&mut tuple));
    munge!(let Foo((a, _)) = &mut value);
    a.set(10);

    let mut mu = MaybeUninit::<Foo>::uninit();
    munge!(let Foo((a, _)) = &mut mu);
    a.write(10);
}
