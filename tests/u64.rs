use enum_primitive::EnumPrimitive;

#[repr(u64)]
#[derive(Debug, EnumPrimitive, PartialEq)]
#[allow(non_camel_case_types)]
enum TestEnum {
    Foo_451 = 451,
    Bar_1235 = 1235,
    Baz_1224 = 1224,
    Baz_1225,
    Baz_13425520088214583806 = 13425520088214583806,
    Biz_44 = 88 / 2,
    Zero = 0,
}

#[test]
fn read_u64() {
    assert_eq!(Some(TestEnum::Zero), TestEnum::read_u64(0));
    assert_eq!(Some(TestEnum::Foo_451), TestEnum::read_u64(451));
    assert_eq!(Some(TestEnum::Bar_1235), TestEnum::read_u64(1235));
    assert_eq!(Some(TestEnum::Baz_1224), TestEnum::read_u64(1224));
    assert_eq!(Some(TestEnum::Baz_1225), TestEnum::read_u64(1225));
    assert_eq!(
        Some(TestEnum::Baz_13425520088214583806),
        TestEnum::read_u64(13425520088214583806)
    );
    assert_eq!(Some(TestEnum::Biz_44), TestEnum::read_u64(44));
    assert_eq!(None, TestEnum::read_u64(45621));
}

#[test]
fn write_u64() {
    assert_eq!(Some(0), (TestEnum::Zero).write_u64());
    assert_eq!(Some(451), (TestEnum::Foo_451).write_u64());
    assert_eq!(Some(1235), (TestEnum::Bar_1235).write_u64());
    assert_eq!(Some(1224), (TestEnum::Baz_1224).write_u64());
    assert_eq!(Some(1225), (TestEnum::Baz_1225).write_u64());
    assert_eq!(
        Some(13425520088214583806),
        (TestEnum::Baz_13425520088214583806).write_u64()
    );
    assert_eq!(Some(44), (TestEnum::Biz_44).write_u64());
}

#[test]
fn failed_read() {
    assert_eq!(None, TestEnum::read_u8(0));
    assert_eq!(None, TestEnum::read_u16(0));
    assert_eq!(None, TestEnum::read_u32(0));
    assert_eq!(Some(TestEnum::Zero), TestEnum::read_u64(0));
    assert_eq!(None, TestEnum::read_u128(0));

    assert_eq!(None, TestEnum::read_i8(0));
    assert_eq!(None, TestEnum::read_i16(0));
    assert_eq!(None, TestEnum::read_i32(0));
    assert_eq!(None, TestEnum::read_i64(0));
    assert_eq!(None, TestEnum::read_i128(0));
}

#[test]
fn failed_write() {
    assert_eq!(None, (TestEnum::Zero).write_u8());
    assert_eq!(None, (TestEnum::Zero).write_u16());
    assert_eq!(None, (TestEnum::Zero).write_u32());
    assert_eq!(Some(0), (TestEnum::Zero).write_u64());
    assert_eq!(None, (TestEnum::Zero).write_u128());

    assert_eq!(None, (TestEnum::Zero).write_i8());
    assert_eq!(None, (TestEnum::Zero).write_i16());
    assert_eq!(None, (TestEnum::Zero).write_i32());
    assert_eq!(None, (TestEnum::Zero).write_i64());
    assert_eq!(None, (TestEnum::Zero).write_i128());
}
