pub use enum_syntax::EnumDeclarationSyntax;
pub use field_syntax::FieldDeclaration;
pub use message_syntax::{MemberDeclaration, MessageDeclarationSyntax};

mod enum_syntax;
mod field_syntax;
mod message_syntax;

#[derive(Debug)]
pub enum DeclarationSyntax {
    Message(MessageDeclarationSyntax),
    Enum(EnumDeclarationSyntax),
}

#[derive(Debug, Eq, PartialEq)]
pub struct ProtocolDeclarationSyntax {
    pub components: Vec<String>,
}

impl ProtocolDeclarationSyntax {
    pub fn new(components: Vec<String>) -> Self {
        ProtocolDeclarationSyntax { components }
    }
}

#[derive(Debug)]
pub struct SyntaxUnit {
    pub protocol: ProtocolDeclarationSyntax,
    pub declarations: Vec<DeclarationSyntax>,
}

impl SyntaxUnit {
    pub fn new(protocol: ProtocolDeclarationSyntax) -> Self {
        SyntaxUnit {
            protocol,
            declarations: vec![],
        }
    }

    pub fn add_declaration(&mut self, decl: DeclarationSyntax) {
        self.declarations.push(decl)
    }
}

#[derive(Debug)]
pub enum Signedness {
    Signed,
    Unsigned,
}

#[derive(Debug)]
pub enum CustomType {
    Enum(EnumDeclarationSyntax),
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
