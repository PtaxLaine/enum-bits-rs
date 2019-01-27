# enum_bits [![pipeline status](https://gitlab.com/PtaxLaine/enum-bits-rs/badges/master/pipeline.svg)](https://gitlab.com/PtaxLaine/enum-bits-rs/commits/master)


## Documentation

...

## Usage

Add dependency into `Cargo.toml` file:

```
[dependencies]
enum_bits = "0.1"
```

Import derive macros to you source with `use enum_bits::EnumBits;`

Add explicit type defination for you enum `#[repr(i32)]`

Add derive to you enum `#[derive(EnumBits)]`

## Example

```rust
use enum_bits::EnumBits;
#[repr(i64)]
#[derive(EnumBits)]
enum TestEnum {
    Foo, Bar, Biz=-57
}

assert_eq!(Some(TestEnum::Foo), TestEnum::read_i64(0));
assert_eq!(Some(TestEnum::Bar), TestEnum::read_i64(1));
assert_eq!(Some(TestEnum::Biz), TestEnum::read_i64(-57));
assert_eq!(None, TestEnum::read_i64(2));
```
