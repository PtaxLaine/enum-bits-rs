#![cfg(nightly)]
#![feature(repr128)]
use enum_primitive::EnumPrimitive;

#[repr(i128)]
#[derive(Debug, EnumPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
enum TestEnum {
    Foo_77 = 77,
    Bar_3689348814741910323 = 3689348814741910323,
    Bar_3689348814741910324,
    Baz_m3689348814741910323 = -3689348814741910323,
    Baz_m3689348814741910322,
    Biz_44 = 88 / 2,
    Zero = 0,
}

#[test]
fn read_i128() {
    assert_eq!(Some(TestEnum::Zero), TestEnum::read_i128(0));
    assert_eq!(Some(TestEnum::Foo_77), TestEnum::read_i128(77));
    assert_eq!(
        Some(TestEnum::Bar_3689348814741910323),
        TestEnum::read_i128(3689348814741910323)
    );
    assert_eq!(
        Some(TestEnum::Bar_3689348814741910324),
        TestEnum::read_i128(3689348814741910324)
    );
    assert_eq!(
        Some(TestEnum::Baz_m3689348814741910323),
        TestEnum::read_i128(-3689348814741910323)
    );
    assert_eq!(
        Some(TestEnum::Baz_m3689348814741910322),
        TestEnum::read_i128(-3689348814741910322)
    );
    assert_eq!(Some(TestEnum::Biz_44), TestEnum::read_i128(44));
    assert_eq!(None, TestEnum::read_i128(33));
}

#[test]
fn write_i128() {
    assert_eq!(Some(0), (TestEnum::Zero).write_i128());
    assert_eq!(Some(77), (TestEnum::Foo_77).write_i128());
    assert_eq!(
        Some(3689348814741910323),
        (TestEnum::Bar_3689348814741910323).write_i128()
    );
    assert_eq!(
        Some(3689348814741910324),
        (TestEnum::Bar_3689348814741910324).write_i128()
    );
    assert_eq!(
        Some(-3689348814741910323),
        (TestEnum::Baz_m3689348814741910323).write_i128()
    );
    assert_eq!(
        Some(-3689348814741910322),
        (TestEnum::Baz_m3689348814741910322).write_i128()
    );
    assert_eq!(Some(44), (TestEnum::Biz_44).write_i128());
}

#[test]
fn failed_read() {
    assert_eq!(None, TestEnum::read_u8(0));
    assert_eq!(None, TestEnum::read_u16(0));
    assert_eq!(None, TestEnum::read_u32(0));
    assert_eq!(None, TestEnum::read_u64(0));
    assert_eq!(None, TestEnum::read_u128(0));

    assert_eq!(None, TestEnum::read_i8(0));
    assert_eq!(None, TestEnum::read_i16(0));
    assert_eq!(None, TestEnum::read_i32(0));
    assert_eq!(None, TestEnum::read_i64(0));
    assert_eq!(Some(TestEnum::Zero), TestEnum::read_i128(0));
}

#[test]
fn failed_write() {
    assert_eq!(None, (TestEnum::Zero).write_u8());
    assert_eq!(None, (TestEnum::Zero).write_u16());
    assert_eq!(None, (TestEnum::Zero).write_u32());
    assert_eq!(None, (TestEnum::Zero).write_u64());
    assert_eq!(None, (TestEnum::Zero).write_u128());

    assert_eq!(None, (TestEnum::Zero).write_i8());
    assert_eq!(None, (TestEnum::Zero).write_i16());
    assert_eq!(None, (TestEnum::Zero).write_i32());
    assert_eq!(None, (TestEnum::Zero).write_i64());
    assert_eq!(Some(0), (TestEnum::Zero).write_i128());
}
