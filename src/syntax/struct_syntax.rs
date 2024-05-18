use crate::syntax::field_syntax::FieldDeclaration;

#[derive(Debug)]
pub struct StructDeclaration {
    pub name: String,
    pub id: Option<usize>,
    pub fields: Vec<FieldDeclaration>,
    pub description: Option<String>,
}
