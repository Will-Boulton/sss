use crate::syntax::field_type_syntax::FieldType;

#[derive(Debug)]
pub struct FieldDeclaration {
    pub name: String,
    pub field_type: FieldType,
    pub description: Option<String>,
}
