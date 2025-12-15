use core::mem::MaybeUninit;
use munge::munge;

fn main() {
    struct Example {
        a: u32,
        b: u32,
    }

    let mut mu = MaybeUninit::<Example>::uninit();

    munge!(let Example { a: a1, b: b1 } = &mut mu);
    assert_eq!(a1.write(1), &1);
    assert_eq!(b1.write(2), &2);

    munge!(let Example { a: a2, b: b2 } = &mut mu);
    assert_eq!(a1.write(3), &3);
    assert_eq!(b1.write(4), &4);

    a2.write(5);
    b2.write(6);
}
