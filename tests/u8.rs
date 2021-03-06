use enum_bits::EnumBits;

#[repr(u8)]
#[derive(Debug, EnumBits, PartialEq)]
#[allow(non_camel_case_types)]
enum TestEnum {
    Foo_77 = 77,
    Bar_141 = 141,
    Bar_142,
    Baz_255 = 255,
    Biz_44 = 88 / 2,
    Zero = 0,
}

#[test]
fn read_u8() {
    assert_eq!(Some(TestEnum::Zero), TestEnum::read_u8(0));
    assert_eq!(Some(TestEnum::Foo_77), TestEnum::read_u8(77));
    assert_eq!(Some(TestEnum::Bar_141), TestEnum::read_u8(141));
    assert_eq!(Some(TestEnum::Bar_142), TestEnum::read_u8(142));
    assert_eq!(Some(TestEnum::Baz_255), TestEnum::read_u8(255));
    assert_eq!(Some(TestEnum::Biz_44), TestEnum::read_u8(44));
    assert_eq!(None, TestEnum::read_u8(33));
}

#[test]
fn write_u8() {
    assert_eq!(Some(0), (TestEnum::Zero).write_u8());
    assert_eq!(Some(77), (TestEnum::Foo_77).write_u8());
    assert_eq!(Some(141), (TestEnum::Bar_141).write_u8());
    assert_eq!(Some(142), (TestEnum::Bar_142).write_u8());
    assert_eq!(Some(255), (TestEnum::Baz_255).write_u8());
    assert_eq!(Some(44), (TestEnum::Biz_44).write_u8());
}

#[test]
fn failed_read() {
    assert_eq!(Some(TestEnum::Zero), TestEnum::read_u8(0));
    assert_eq!(None, TestEnum::read_u16(0));
    assert_eq!(None, TestEnum::read_u32(0));
    assert_eq!(None, TestEnum::read_u64(0));
    assert_eq!(None, TestEnum::read_u128(0));

    assert_eq!(None, TestEnum::read_i8(0));
    assert_eq!(None, TestEnum::read_i16(0));
    assert_eq!(None, TestEnum::read_i32(0));
    assert_eq!(None, TestEnum::read_i64(0));
    assert_eq!(None, TestEnum::read_i128(0));
}

#[test]
fn failed_write() {
    assert_eq!(Some(0), (TestEnum::Zero).write_u8());
    assert_eq!(None, (TestEnum::Zero).write_u16());
    assert_eq!(None, (TestEnum::Zero).write_u32());
    assert_eq!(None, (TestEnum::Zero).write_u64());
    assert_eq!(None, (TestEnum::Zero).write_u128());

    assert_eq!(None, (TestEnum::Zero).write_i8());
    assert_eq!(None, (TestEnum::Zero).write_i16());
    assert_eq!(None, (TestEnum::Zero).write_i32());
    assert_eq!(None, (TestEnum::Zero).write_i64());
    assert_eq!(None, (TestEnum::Zero).write_i128());
}
