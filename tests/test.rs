use sss::lexer::Keyword;
use sss::lexer::TokenType;
use sss::lexer::*;

macro_rules! assert_no_token {
    ($tokens:ident) => {{
        let next = $tokens.next();
        assert_eq!(
            next, None,
            "token '{:?}' found when none were expected",
            next
        );
    }};
}

macro_rules! assert_next_token {
    ($tokens:ident,$token:expr) => {{
        let next = $tokens.next();
        assert_ne!(next, None, "no token found, expected '{:?}'", $token);
        assert_eq!(next, Some($token));
    }};
}

macro_rules! assert_token_types {
    ($tokens:ident) => {
        assert_no_token!($tokens);
    };
    ($tokens:ident,$token:expr) => {
        assert_next_token!($tokens, $token);
        assert_no_token!($tokens);
    };
    ($tokens:ident,$token:expr, $($expected:expr),*) => {
        assert_next_token!($tokens, $token);
        assert_token_types!($tokens, $($expected),*);
    };
    ($text:literal,
        $($expected:expr),*) => {
        let mut tokens = tokenize($text).map(|t|t.get().clone());
        assert_token_types!(tokens, $($expected),*);
    };
}

macro_rules! loc {
    ($l:literal: $c:literal) => {
        SourcePoint::new($l, $c)
    };
}

macro_rules! range {
    ($l1:literal: $c1:literal > $l2:literal: $c2:literal) => {
        SourceRange::new([$l1, $c1], [$l2, $c2])
    };
}

macro_rules! identifier {
    ($i:ident[$r:expr]) => {
        TokenType::Identifier($r, String::from(stringify!($i)))
    };
}

macro_rules! token {
    (:) => {
        TokenType::Colon
    };
    (,) => {
        TokenType::Comma
    };
    (;) => {
        TokenType::SemiColon
    };
    (obkt) => {
        TokenType::OpenBracket
    };
    (cbkt) => {
        TokenType::CloseBracket
    };
    (obra) => {
        TokenType::OpenBrace
    };
    (cbra) => {
        TokenType::CloseBrace
    };
    (kw: $kw:expr) => {
        TokenType::Keyword($kw)
    };
    (id: $id:ident) => {
        TokenType::Identifier(String::from(stringify!($id)))
    };
    (num: $num:literal ) => {
        TokenType::IntegerLiteral(String::from(stringify!($num)))
    };
}

#[test]
fn test_token_macro() {
    assert_eq!(token!(:), TokenType::Colon);

    assert_eq!(token!(,), TokenType::Comma);

    assert_eq!(token!(;), TokenType::SemiColon);
    assert_eq!(token!(id: foo), TokenType::Identifier(String::from("foo")));
}

#[test]
fn test_lex_identifier() {
    assert_token_types!("a", token!(id: a));
}

#[test]
fn test_lex_colon_after_identifier() {
    assert_token_types!("id:   ", token!(id: id), token!(:));
}

#[test]
fn test_lex_locations_valid_with_whitespace() {
    assert_token_types!("    id :", token!(id: id), token!(:));
}

#[test]
fn test_lexer() {
    assert_token_types!(
        "an_identifier 123 [ ] ;
:foobert421_fasga:
, { }",
        token!(id: an_identifier),
        token!(num: 123),
        token!(obkt),
        token!(cbkt),
        token!(;),
        token!(:),
        token!(id: foobert421_fasga),
        token!(:),
        token!(,),
        token!(obra),
        token!(cbra)
    );
}

#[test]
fn test_lex_type() {
    assert_token_types!(
        "struct foo {
    uint32 id;
    uint8  flags;
    bool   enabled;
} ",
        token!(kw: Keyword::Struct),
        token!(id: foo),
        token!(obra),
        token!(id: uint32),
        token!(id: id),
        token!(;),
        token!(id: uint8),
        token!(id: flags),
        token!(;),
        token!(id: bool),
        token!(id: enabled),
        token!(;),
        token!(cbra)
    );
}
