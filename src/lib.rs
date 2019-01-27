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
