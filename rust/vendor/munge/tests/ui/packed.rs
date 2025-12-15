use core::mem::MaybeUninit;
use munge::munge;

fn main() {
    #[repr(packed)]
    struct Misalign<T> {
        byte: u8,
        inner: T,
    }

    let mut mu = MaybeUninit::<Misalign<Misalign<u32>>>::uninit();

    munge!(
        let Misalign { byte: a, inner: Misalign { byte: b, inner } } = &mut mu;
    );
    assert_eq!(a.write(1), &1);
    assert_eq!(b.write(2), &2);
    assert_eq!(inner.write(3), &3);

    // SAFETY: `mu` is completely initialized.
    let init = unsafe { mu.assume_init() };
    assert_eq!(init.byte, 1);
    assert_eq!(init.inner.byte, 2);
    assert_eq!(init.inner.inner, 3);
}
