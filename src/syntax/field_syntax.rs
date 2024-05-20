use crate::data_types::FieldType_;

#[derive(Debug)]
pub struct FieldDeclaration {
    pub name: String,
    pub field_type: FieldType_,
    pub description: Option<String>,
}
