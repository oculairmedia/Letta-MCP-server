macro_rules! match_endian {
    (little $little:expr, $big:expr $(,)?) => {
        $little
    };
    (big $little:expr, $big:expr $(,)?) => {
        $big
    };
}

macro_rules! if_native_endian {
    ($endian:ident $true:expr, $false:expr $(,)?) => {
        match_endian!(
            $endian
            {
                #[cfg(target_endian = "little")]
                {
                    $true
                }
                #[cfg(target_endian = "big")]
                {
                    $false
                }
            },
            {
                #[cfg(target_endian = "little")]
                {
                    $false
                }
                #[cfg(target_endian = "big")]
                {
                    $true
                }
            },
        )
    }
}

macro_rules! swap_endian {
    ($endian:ident $expr:expr) => {
        if_native_endian!($endian $expr, $expr.swap_bytes())
    }
}

macro_rules! endian_name {
    ($endian:ident) => {
        match_endian!($endian "little", "big")
    };
}

#[cfg(any(
    target_has_atomic = "16",
    target_has_atomic = "32",
    target_has_atomic = "64",
))]
macro_rules! opposite_endian_name {
    ($endian:ident) => {
        match_endian!($endian "big", "little")
    };
}

#[cfg(test)]
macro_rules! assert_size_align {
    ($($name:ident $size:literal $align:literal),* $(,)?) => {
        $(
            assert_eq!(core::mem::size_of::<$name>(), $size);
            assert_eq!(core::mem::align_of::<$name>(), $align);
        )*
    }
}
