use crate::syntax::{BuiltInType, EnumValue};

#[derive(Debug)]
pub struct EnumDeclarationSyntax {
    pub name: String,
    pub underlying_type: BuiltInType,
    pub value: Vec<EnumValue>,
}
