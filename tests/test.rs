
use really_simple_type_specs as rss;
use really_simple_type_specs::source::SourceLocation;
use rss::source::SourceRange;

use rss::lexer::*;
use rss::lexer::Token;

macro_rules! assert_no_token {
    ($tokens:ident) => {
        {
            let next = $tokens.next();
            assert_eq!(next, None, "token '{:?}' found when none were expected", next);
        }

    }
}

macro_rules! assert_next_token {
    ($tokens:ident,$token:expr) => {
        {
            let next = $tokens.next();
            assert_ne!(next,None, "no token found, expected '{:?}'", $token);
            assert_eq!(next, Some($token));
        }

    };
}

macro_rules! assert_tokens{
    ($tokens:ident) => {
        assert_no_token!($tokens);
    };
    ($tokens:ident,$token:expr) => {
        assert_next_token!($tokens, $token);
        assert_no_token!($tokens);
    };
    ($tokens:ident,$token:expr, $($expected:expr),*) => {
        assert_next_token!($tokens, $token);
        assert_tokens!($tokens, $($expected),*);
    };
    ($text:literal,
        $($expected:expr),*) => {
        let mut tokens = tokenize($text);
        assert_tokens!(tokens, $($expected),*);
    };
}

macro_rules! loc {
    ($l:literal: $c:literal) => {
        SourceLocation::new($l,$c)
    };
}


macro_rules! range {
    ($l1:literal: $c1:literal > $l2:literal: $c2:literal) => {
        SourceRange::new([$l1,$c1], [$l2, $c2])
    };
}

macro_rules! identifier {
    ($i:ident[$r:expr]) => {
        Token::Identifier($r,String::from(stringify!($i)))
    };
}

macro_rules! token {
    ($t:ident, $l:literal, $c:literal) => {
        Token::$t(loc!($l:$c))
    };
    (: @ $l:literal: $c:literal) => {
        token!(Colon,$l,$c)
    };
    (, @ $l:literal: $c:literal) => {
        token!(Comma,$l,$c)
    };
    (; @ $l:literal: $c:literal) => {
        token!(SemiColon,$l,$c)
    };
    (obkt @ $l:literal: $c:literal) => {
        token!(OpenBracket,$l,$c)
    };
    (cbkt @ $l:literal: $c:literal) => {
        token!(CloseBracket,$l,$c)
    };
    (obra @ $l:literal: $c:literal) => {
        token!(OpenBrace,$l,$c)
    };
    (cbra @ $l:literal: $c:literal) => {
        token!(CloseBrace,$l,$c)
    };
    (id: $id:literal @ [$r:expr]) => {
        Token::Identifier($r, String::from($id))
    };
    (num: $num:literal @ [$r:expr]) => {
        Token::IntegerLiteral($r, String::from(stringify!($num)))
    };
}

#[test]
fn test_token_macro()
{
    assert_eq!(
        token!(: @0:2),
        Token::Colon(SourceLocation::new(0,2))
    );

    assert_eq!(
        token!(, @110:22),
        Token::Comma(SourceLocation::new(110,22))
    );

    assert_eq!(
        token!(; @5:1222),
        Token::SemiColon(SourceLocation::new(5,1222))
    );
}

#[test]
fn test_lex_identifier()
{
    assert_tokens!( "a",
       token!(id: "a" @ [range!(0:0>0:0)]) ,
       token!(: @0:2)
    );
}

#[test]
fn test_lex_colon_after_identifier()
{
    assert_tokens!( "id:   ",
       token!(id: "id" @ [range!(0:0>0:2)]) ,
       token!(: @0:2)
    );
}

#[test]
fn test_lex_locations_valid_with_whitespace()
{
    assert_tokens!( "    id :",
       identifier!(id[range!(0:4>0:6)]) ,
       token!(: @0:7)
    );
}

#[test]
fn test_lexer() {
    assert_tokens!( "an_identifier 123 [ ] ;
foobert421_fasga 
, { }",
       token!(id: "an_identifier" @ [range!(0:0>0:13)]),
       token!(num: 123 @ [range!(0:14>0:17)]),
       token!(obkt @ 0:18),
       token!(cbkt @ 0:20),
       token!(; @ 0:22),
       token!(id: "foobert421_fasga" @ [range!(1:0>1:16)])
    );
}

#[test]
fn test_lex_type() {
    assert_tokens!(
"struct foo {
    uint32 id;
    uint8  flags;
    bool   enabled;
} ",
        token!(id: "struct" @ [range!(0:0>0:6)]),
        token!(id: "foo" @ [range!(0:7>0:10)]),
        token!(obra @ 0:11),
        token!(id: "uint32" @ [range!(0:0>0:13)]),
        token!(id: "id" @ [range!(0:0>0:13)]),
        token!(; @ 1:1),
        token!(id: "uint8" @ [range!(0:0>0:13)]),
        token!(id: "flags" @ [range!(0:0>0:13)]),
        token!(; @ 1:1),
        token!(id: "bool" @ [range!(0:0>0:13)]),
        token!(id: "enabled" @ [range!(0:0>0:13)]),
        token!(; @ 1:1),
        token!(cbra @ 1:1)
    );
}