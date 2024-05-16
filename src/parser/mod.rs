pub enum SyntaxNode {
    Struct(StructDeclaration),
}

#[derive(Debug)]
pub enum StructLayout {
    Auto,
    Explicit,
}

#[derive(Debug)]
pub struct StructDeclaration {
    pub name: String,
    pub id: Option<usize>,
    pub fields: Vec<FieldDeclaration>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub enum Signedness {
    Signed,
    Unsigned,
}

#[derive(Debug)]
pub struct FieldDeclaration {
    pub name: String,
    pub field_type: FieldType,
    pub description: Option<String>,
}

#[derive(Debug)]
pub enum FieldType {
    BuiltIn(BuiltInType),
    Custom(CustomType),
}

#[derive(Debug)]
pub enum CustomType {
    Enum(EnumDeclaration),
}

#[derive(Debug)]
pub struct EnumDeclaration {
    pub name: String,
    pub underlying_type: BuiltInType,
    pub value: Vec<EnumValue>,
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
