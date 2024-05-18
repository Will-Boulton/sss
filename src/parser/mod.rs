use crate::lexer::{Keyword, Token, TokenType};
use crate::parser::ParseError::UnexpectedToken;
use crate::syntax::{ProtocolDeclarationSyntax, SyntaxUnit};

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token),
    ExpectedAToken,
    MissingIdentifier,
    ExpectedProtocolDeclaration,
}

pub struct Parser<'a, T: Iterator<Item = Token> + Clone> {
    tokens: &'a mut T,
}

pub fn parse<T: Iterator<Item = Token> + Clone>(
    tokens: &mut T,
) -> Result<Option<SyntaxUnit>, ParseError> {
    Parser::new(tokens).parse()
}

impl<'a, T> Parser<'a, T>
where
    T: Iterator<Item = Token> + Clone,
{
    pub fn new(tokens: &'a mut T) -> Self {
        Parser { tokens }
    }
    pub fn parse(&mut self) -> Result<Option<SyntaxUnit>, ParseError> {
        let protocol = self.parse_protocol()?;
        Ok(Some(SyntaxUnit::new(protocol)))
    }

    pub fn parse_protocol(&mut self) -> Result<ProtocolDeclarationSyntax, ParseError> {
        return match self.tokens.next() {
            Some(token) => match token.get_type() {
                TokenType::Keyword(keyword) if keyword == &Keyword::Protocol => {
                    match self.parse_qualified_name() {
                        Ok(pds) => {
                            self.assert_next_token_matches(TokenType::SemiColon)?;
                            Ok(ProtocolDeclarationSyntax::new(pds))
                        }
                        Err(e) => Err(e),
                    }
                }
                _ => Err(ParseError::UnexpectedToken(token)),
            },
            None => Err(ParseError::ExpectedProtocolDeclaration),
        };
    }

    pub fn assert_next_token_matches(&mut self, tt: TokenType) -> Result<(), ParseError> {
        return match self.tokens.next() {
            Some(token) => {
                if token.get_type() == &tt {
                    return Ok(());
                }
                Err(UnexpectedToken(token))
            }
            None => Err(ParseError::ExpectedAToken),
        };
    }

    pub fn parse_qualified_name(&mut self) -> Result<Vec<String>, ParseError> {
        let mut dot_expected = false;
        let qualified_name_parts: Vec<_> = self
            .tokens
            .clone()
            .take_while(|t| match t.get_type() {
                TokenType::Identifier(_) if !dot_expected => {
                    dot_expected = true;
                    true
                }
                TokenType::Dot if dot_expected => {
                    dot_expected = false;
                    true
                }
                _ => false,
            })
            .collect();

        for _ in 0..qualified_name_parts.len() {
            _ = self.tokens.next()
        }

        if let Some(x) = qualified_name_parts.last() {
            if x.get_type() == &TokenType::Dot {
                return Err(UnexpectedToken(x.clone()));
            };
        };

        return Ok(qualified_name_parts
            .iter()
            .filter_map(|t| match t.get_type() {
                TokenType::Identifier(txt) => Some(txt.clone()),
                _ => None,
            })
            .collect());
    }
}
