pub use enum_syntax::EnumDeclaration;
pub use field_syntax::FieldDeclaration;
pub use field_type_syntax::FieldType;
pub use struct_syntax::StructDeclaration;

mod enum_syntax;
mod field_syntax;
mod field_type_syntax;
mod struct_syntax;

pub enum SyntaxNode {
    Struct(StructDeclaration),
}

#[derive(Debug)]
pub enum Signedness {
    Signed,
    Unsigned,
}

#[derive(Debug)]
pub enum CustomType {
    Enum(EnumDeclaration),
}

#[derive(Debug)]
pub struct EnumValue {
    name: String,
    value: String,
}
#[derive(Debug)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}
#[derive(Debug)]
pub enum BuiltInType {
    Integer {
        size_bits: usize,
        signedness: Signedness,
        maybe_endianness: Option<Endianness>,
    },
    AsciiFixedString {
        length: usize,
    },
}

#[derive(Debug)]
pub enum FixedStringType {
    Ascii { length_bytes: usize },
}
