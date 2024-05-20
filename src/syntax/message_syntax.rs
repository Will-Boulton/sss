use crate::syntax::field_syntax::FieldDeclaration;

#[derive(Debug)]
pub struct MessageDeclarationSyntax {
    pub name: String,
    pub id: Option<usize>,
    pub fields: Vec<FieldDeclaration>,
}
