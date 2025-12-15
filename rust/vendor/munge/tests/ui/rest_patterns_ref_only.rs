use core::mem::MaybeUninit;
use munge::munge;

fn main() {
    rest_struct();
    rest_tuple();
}

fn rest_struct() {
    struct Struct {
        a: u32,
        b: u32,
    }

    let mut mu = MaybeUninit::<Struct>::uninit();

    munge!(let Struct { a, .. } = mu);
}

fn rest_tuple() {
    struct Tuple(u32, u32);

    let mut mu = MaybeUninit::<Tuple>::uninit();

    munge!(let Tuple(a, ..) = mu);
}
