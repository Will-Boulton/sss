use crate::lexer::{Token, TokenType};
use crate::parser::ParseError::UnexpectedToken;
use crate::syntax::{
    DeclarationSyntax, MessageDeclarationSyntax, ProtocolDeclarationSyntax, SyntaxUnit,
};

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token, Option<String>),
    Err(String),
    ExpectedAToken,
    MissingIdentifier,
    ExpectedProtocolDeclaration,
    InvalidNumberFormat
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
        self.parse_protocol()
            .and_then(|pds| Ok(SyntaxUnit::new(pds)))
            .and_then(|mut syntax_unit| {
                loop {
                    match self.parse_declaration() {
                        Ok(Some(decl)) => {
                            syntax_unit.add_declaration(decl);
                        }
                        Ok(None) => break,
                        Err(E) => return Err(E),
                    }
                }
                return Ok(Some(syntax_unit));
            })
    }

    pub fn parse_declaration(&mut self) -> Result<Option<DeclarationSyntax>, ParseError> {
        return self.get_identifier().and_then(|maybe_id| match maybe_id {
            None => Ok(None),
            Some(id) if id == "message" => {
                return self
                    .parse_message_declaration()
                    .and_then(move |m| Ok(Some(DeclarationSyntax::Message(m))))
            }
            Some(id) if id == "enum" => {
                todo!()
            }
            Some(id) if id == "bitflags" => {
                todo!()
            }
            Some(id) => Err(ParseError::Err(String::from(format!(
                "Unexpected identifier '{}', expected one of {{message, enum, bitflags}}",
                id
            )))),
        });
    }

    fn parse_message_declaration(&mut self) -> Result<MessageDeclarationSyntax, ParseError> {
        self.assert_next_token_matches(TokenType::OpenBracket)?;
        let id = self.parse_number()?;

        self.assert_next_token_matches(TokenType::CloseBracket)?;
        self.assert_next_token_matches(TokenType::OpenBrace)?;

        let fields = vec![];



        self.assert_next_token_matches(TokenType::CloseBrace)?;
    }

    fn parse_number(&mut self) -> Result<usize, ParseError> {
        match self.tokens.next() {
            None => Err(ParseError::ExpectedAToken),
            Some(t) => match t.get_type() {
                TokenType::IntegerLiteral(int) => match int.parse::<usize>() {
                    Ok(val) => Ok(val),
                    Err(_) => Err(ParseError::InvalidNumberFormat)
                },
                _ => Err(UnexpectedToken(t.clone(),None))
            }
        }
    }

    pub fn assert_next_token_matches(&mut self, tt: TokenType) -> Result<Token, ParseError> {
        return match self.tokens.next() {
            Some(token) => {
                if token.get_type() == &tt {
                    return Ok(token);
                }
                Err(UnexpectedToken(token, None))
            }
            None => Err(ParseError::ExpectedAToken),
        };
    }

    fn assert_next_token<F: FnOnce(&Token) -> bool>(
        &mut self,
        token_matches: F,
    ) -> Result<Token, Option<Token>> {
        match self.tokens.next() {
            None => Err(None),
            Some(token) if token_matches(&token) => Ok(token.clone()),
            Some(token) => Err(Some(token)),
        }
    }

    fn get_identifier(&mut self) -> Result<Option<String>, ParseError> {
        return match self.tokens.next() {
            Some(token) => match token.get_type() {
                TokenType::Identifier(id) => Ok(Some(id.clone())),
                _ => Err(UnexpectedToken(
                    token,
                    Some(String::from("Expected an identifier")),
                )),
            },
            None => Ok(None),
        };
    }

    pub fn parse_protocol(&mut self) -> Result<ProtocolDeclarationSyntax, ParseError> {
        return match self.tokens.next() {
            Some(token) => match token.get_type() {
                TokenType::Identifier(id) if id == "protocol" => {
                    self.parse_qualified_name().and_then(|pds| {
                        self.assert_next_token_matches(TokenType::SemiColon)?;
                        Ok(ProtocolDeclarationSyntax::new(pds))
                    })
                }
                _ => Err(UnexpectedToken(token, None)),
            },
            None => Err(ParseError::ExpectedProtocolDeclaration),
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
                return Err(UnexpectedToken(x.clone(), None));
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
