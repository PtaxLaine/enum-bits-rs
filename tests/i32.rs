use enum_primitive::EnumPrimitive;

#[repr(i32)]
#[derive(Debug, EnumPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
enum TestEnum {
    Foo_77 = 77,
    Bar_88 = 88,
    Baz_m88 = -88,
    Biz_44 = 88 / 2,
    Zero = 0,
}

#[test]
fn read_i32() {
    assert_eq!(Some(TestEnum::Zero), TestEnum::read_i32(0));
    assert_eq!(Some(TestEnum::Foo_77), TestEnum::read_i32(77));
    assert_eq!(Some(TestEnum::Bar_88), TestEnum::read_i32(88));
    assert_eq!(Some(TestEnum::Baz_m88), TestEnum::read_i32(-88));
    assert_eq!(Some(TestEnum::Biz_44), TestEnum::read_i32(44));
    assert_eq!(None, TestEnum::read_i32(33));
}

#[test]
fn write_i32() {
    assert_eq!(Some(0), (TestEnum::Zero).write_i32());
    assert_eq!(Some(77), (TestEnum::Foo_77).write_i32());
    assert_eq!(Some(88), (TestEnum::Bar_88).write_i32());
    assert_eq!(Some(-88), (TestEnum::Baz_m88).write_i32());
    assert_eq!(Some(44), (TestEnum::Biz_44).write_i32());
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
    assert_eq!(Some(TestEnum::Zero), TestEnum::read_i32(0));
    assert_eq!(None, TestEnum::read_i64(0));
    assert_eq!(None, TestEnum::read_i128(0));
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
    assert_eq!(Some(0), (TestEnum::Zero).write_i32());
    assert_eq!(None, (TestEnum::Zero).write_i64());
    assert_eq!(None, (TestEnum::Zero).write_i128());
}
