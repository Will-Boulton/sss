use crate::source::{SourceLocation, SourcePoint, SourceRange};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Keyword {
    Message,
    Protocol,
    Reserved,
    Struct,
}

pub(super) fn lex_keyword(txt: &str) -> Option<Keyword> {
    return match txt {
        "message" => Some(Keyword::Message),
        "protocol" => Some(Keyword::Protocol),
        "reserved" => Some(Keyword::Reserved),
        "struct" => Some(Keyword::Struct),
        &_ => None,
    };
}

pub struct Token {
    token: TokenType,
    location: SourceLocation,
}

#[macro_export]
macro_rules! token {
    ($tt:ident, $loc:expr) => {
        Token::new(TokenType::$tt, $loc)
    };
    ($tt:ident, $loc:expr $(,$args:expr)+) => {
        Token::new(TokenType::$tt($($args),+), $loc)
    };
}

impl Token {
    pub fn get(&self) -> &TokenType {
        return &self.token;
    }

    pub fn from_point(token: TokenType, source_point: SourcePoint) -> Self {
        return Token {
            token,
            location: SourceLocation::Point(source_point),
        };
    }
    pub fn from_range(token: TokenType, source_range: SourceRange) -> Self {
        return Token {
            token,
            location: SourceLocation::Range(source_range),
        };
    }

    pub fn new(token: TokenType, location: SourceLocation) -> Self {
        return Token { token, location };
    }
}
