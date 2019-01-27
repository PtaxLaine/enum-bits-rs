//! # enum_bits [![pipeline status](https://gitlab.com/PtaxLaine/enum-bits-rs/badges/master/pipeline.svg)](https://gitlab.com/PtaxLaine/enum-bits-rs/commits/master)
//!
//!
//! ## Documentation
//!
//! ...
//!
//! ## Usage
//!
//! Add dependency into `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! enum_bits = "0.1"
//! ```
//!
//! Import derive macros to you source with `use enum_bits::EnumBits;`
//!
//! Add explicit type defination for you enum `#[repr(i32)]`
//!
//! Add derive to you enum `#[derive(EnumBits)]`
//!
//! ## Example
//!
//! ```rust
//! use enum_bits::EnumBits;
//!
//! #[repr(i64)]
//! #[derive(EnumBits)]
//! #[derive(Debug, PartialEq)] // assert_eq required
//! enum TestEnum {
//!     Foo, Bar, Biz=-57
//! }
//!
//! assert_eq!(Some(TestEnum::Foo), TestEnum::read_i64(0));
//! assert_eq!(Some(TestEnum::Bar), TestEnum::read_i64(1));
//! assert_eq!(Some(TestEnum::Biz), TestEnum::read_i64(-57));
//! assert_eq!(None, TestEnum::read_i64(2));
//! ```

pub use enum_bits_derive::EnumBits;
pub use lazy_static::lazy_static;

pub trait EnumBits: Sized {
    fn read_u8(_i: u8) -> Option<Self> {
        None
    }
    fn read_u16(_i: u16) -> Option<Self> {
        None
    }
    fn read_u32(_i: u32) -> Option<Self> {
        None
    }
    fn read_u64(_i: u64) -> Option<Self> {
        None
    }
    fn read_u128(_i: u128) -> Option<Self> {
        None
    }

    fn read_i8(_i: i8) -> Option<Self> {
        None
    }
    fn read_i16(_i: i16) -> Option<Self> {
        None
    }
    fn read_i32(_i: i32) -> Option<Self> {
        None
    }
    fn read_i64(_i: i64) -> Option<Self> {
        None
    }
    fn read_i128(_i: i128) -> Option<Self> {
        None
    }

    fn write_u8(&self) -> Option<u8> {
        None
    }
    fn write_u16(&self) -> Option<u16> {
        None
    }
    fn write_u32(&self) -> Option<u32> {
        None
    }
    fn write_u64(&self) -> Option<u64> {
        None
    }
    fn write_u128(&self) -> Option<u128> {
        None
    }

    fn write_i8(&self) -> Option<i8> {
        None
    }
    fn write_i16(&self) -> Option<i16> {
        None
    }
    fn write_i32(&self) -> Option<i32> {
        None
    }
    fn write_i64(&self) -> Option<i64> {
        None
    }
    fn write_i128(&self) -> Option<i128> {
        None
    }
}
