use crate::source::{SourceLocation, SourceRange};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Illegal(SourceLocation),
    Identifier(SourceRange, String),
    IntegerLiteral(SourceRange, String),
    SemiColon(SourceLocation),
    Colon(SourceLocation),
    Comma(SourceLocation),
    OpenBrace(SourceLocation),
    CloseBrace(SourceLocation),
    OpenBracket(SourceLocation),
    CloseBracket(SourceLocation),
}
