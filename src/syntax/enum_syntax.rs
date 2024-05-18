use crate::syntax::{BuiltInType, EnumValue};

#[derive(Debug)]
pub struct EnumDeclaration {
    pub name: String,
    pub underlying_type: BuiltInType,
    pub value: Vec<EnumValue>,
}
