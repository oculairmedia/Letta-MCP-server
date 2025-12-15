use core::mem::MaybeUninit;
use munge::munge;

fn main() {
    struct Example {
        a: u32,
        b: u32,
    }

    let mut mu = MaybeUninit::<Example>::uninit();

    munge!(let Example { a, b } = &mut mu);
    assert_eq!(a.write(1), &1);
    assert_eq!(b.write(2), &2);

    // SAFETY: `mu` is completely initialized.
    let value = unsafe { mu.assume_init() };
    assert_eq!(value.a, 1);
    assert_eq!(value.b, 2);

    a.write(3);
}
