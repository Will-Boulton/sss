use crate::syntax::field_syntax::FieldDeclaration;

#[derive(Debug)]
pub struct MessageDeclarationSyntax {
    pub name: String,
    pub id: usize,
    pub members: Vec<MemberDeclaration>,
}

#[derive(Debug)]
pub enum MemberDeclaration {
    Field(FieldDeclaration),
    Padding(usize)
}