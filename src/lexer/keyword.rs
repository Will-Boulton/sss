#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Keyword {
    Enum,
    Message,
    Protocol,
    Reserved,
    Struct,
}

pub(in crate::lexer) fn lex_keyword(txt: &str) -> Option<Keyword> {
    return match txt {
        "enum" => Some(Keyword::Enum),
        "message" => Some(Keyword::Message),
        "protocol" => Some(Keyword::Protocol),
        "reserved" => Some(Keyword::Reserved),
        "struct" => Some(Keyword::Struct),
        &_ => None,
    };
}
