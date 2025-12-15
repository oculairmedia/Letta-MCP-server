# `ptr_meta`

[![crates.io badge]][crates.io] [![docs badge]][docs] [![license badge]][license]

[crates.io badge]: https://img.shields.io/crates/v/ptr_meta.svg
[crates.io]: https://crates.io/crates/ptr_meta
[docs badge]: https://img.shields.io/docsrs/ptr_meta
[docs]: https://docs.rs/ptr_meta
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg
[license]: https://github.com/rkyv/ptr_meta/blob/master/LICENSE

A radioactive stabilization of the [`ptr_meta` RFC].

[`ptr_meta` RFC]: https://rust-lang.github.io/rfcs/2580-ptr-meta.html

## Documentation

- [ptr_meta](https://docs.rs/ptr_meta), the core library
- [ptr_meta_derive](https://docs.rs/ptr_meta_derive), proc macros for
  implementing `Pointee` for structs and trait objects

## Example

```rust
// Get the associated metadata for pointers
let str = "hello world";
assert_eq!(ptr_meta::metadata(str), str.len());

let slice = &[1, 2, 3, 4, 5] as &[i32];
assert_eq!(ptr_meta::metadata(slice), slice.len());

// Make your own wide pointers from data pointers and metadata
let bytes = [b'h', b'e', b'l', b'l', b'o'];
let ptr = ptr_meta::from_raw_parts::<str>(bytes.as_ptr().cast(), 5);
println!("{} world!", unsafe { &*ptr }); // prints "hello world!"

// Derive Pointee on your own types
#[derive(ptr_meta::Pointee)]
#[repr(transparent)]
struct CoolStr {
    inner: str,
}

impl CoolStr {
    fn print_cool(&self) {
        println!("😎 {} 😎", &self.inner);
    }
}

let ptr = ptr_meta::from_raw_parts::<CoolStr>(bytes.as_ptr().cast(), 5);
let cool = unsafe { &*ptr };
cool.print_cool(); // prints "😎 hello 😎"

// Implement Pointee for trait objects
#[ptr_meta::pointee]
trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("i32: {self}");
    }
}

let i32_vtable = ptr_meta::metadata(&0i32 as &dyn Printable);
let one_hundred = 100i32;
let printable = ptr_meta::from_raw_parts::<dyn Printable>(
    (&one_hundred as *const i32).cast(),
    i32_vtable,
);
unsafe {
    (*printable).print(); // prints "i32: 100"
}
```
