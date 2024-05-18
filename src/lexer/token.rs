use crate::lexer::keyword::Keyword;
use crate::source::{SourceLocation, ToLocation};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenType {
    Illegal,
    Identifier(String),
    Keyword(Keyword),
    IntegerLiteral(String),
    SemiColon,
    Colon,
    Comma,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
}

pub struct Token {
    token: TokenType,
    location: SourceLocation,
}

#[macro_export(crate)]
macro_rules! token {
    ($tt:ident, $loc:expr) => {
        Token::new(TokenType::$tt, $loc)
    };
    ($tt:ident, $loc:expr $(,$args:expr)+) => {
        Token::new(TokenType::$tt($($args),+), $loc)
    };
}

impl Token {
    pub fn get_type(&self) -> &TokenType {
        &self.token
    }

    pub fn get_location(&self) -> &SourceLocation {
        &self.location
    }

    pub fn new(token: TokenType, location: impl ToLocation) -> Self {
        return Token {
            token,
            location: location.to_location(),
        };
    }
}
