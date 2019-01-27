use enum_primitive::EnumPrimitive;

#[repr(u16)]
#[derive(Debug, EnumPrimitive, Clone)]
enum TestEnum {
    Foo_451 = 451,
    Bar_1235 = 1235,
    Baz_1224 = 1224,
    Biz_44 = 88 / 2,
    Zero = 0,
}

#[test]
fn main() {
    dbg!(TestEnum::read_u16(0));
    dbg!(TestEnum::read_u16(451));
    dbg!(TestEnum::read_u16(1235));
    dbg!(TestEnum::read_u16(1224));
    dbg!(TestEnum::read_u16(7777));
    dbg!(TestEnum::read_u16(44));
}
