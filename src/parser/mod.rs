use std::cmp::PartialEq;
use crate::data_types::{FieldType_, ScalarType, ArrayLike};
use crate::data_types::scalar::ByteSize;
use crate::lexer::{Token, TokenType};
use crate::parser::ParseError::UnexpectedToken;
use crate::syntax::{DeclarationSyntax, MessageDeclarationSyntax, MemberDeclaration, ProtocolDeclarationSyntax, SyntaxUnit, FieldDeclaration};

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    UnexpectedToken(Token, Option<String>),
    Err(String),
    ExpectedAToken,
    MissingIdentifier,
    ExpectedProtocolDeclaration,
    InvalidNumberFormat,
    UnknownType
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
        let name = self.get_next_token_if(|t| match t.get_type() {
            TokenType::Identifier(id) => Some(id.clone()),
            _ => None
        }).map_err(|t|match t {
            Some(t) => UnexpectedToken(t.clone(), None),
            None => ParseError::ExpectedAToken
        })?;

        self.assert_next_token_matches(TokenType::OpenBracket)?;
        let id = self.parse_number()?;

        self.assert_next_token_matches(TokenType::CloseBracket)?;
        self.assert_next_token_matches(TokenType::OpenBrace)?;

        let mut members : Vec<MemberDeclaration> = vec![];
        while let Some(member) = self.parse_member()? {
            members.push(member);
            if let Some(t) = self.tokens.clone().next()  {
                if *t.get_type() == TokenType::CloseBrace {
                    break
                }
            }
        }
        self.tokens.next();

        return Ok(MessageDeclarationSyntax{ name, id, members })
    }

    fn parse_number(&mut self) -> Result<usize, ParseError> {
        match self.tokens.next() {
            None => Err(ParseError::ExpectedAToken),
            Some(t) => match t.get_type() {
                TokenType::IntegerLiteral(int) => Ok(*int),
                _ => Err(UnexpectedToken(t.clone(),None))
            }
        }
    }

    fn parse_member(&mut self) -> Result<Option<MemberDeclaration>, ParseError> {
        match self.tokens.next() {
            None => Ok(None),
            Some(t) => match t.get_type() {
                TokenType::Identifier(type_name) => match self.try_parse_field(type_name.as_str())? {
                    None => Ok(None),
                    Some(f) => Ok(Some(MemberDeclaration::Field(f)))
                },
                TokenType::IntegerLiteral(size,) => {
                    // only padding can start with an integer literal it will always be followed by
                    // a semi colon
                    self.assert_next_token_matches(TokenType::SemiColon)?;
                    Ok(Some(MemberDeclaration::Padding(*size)))
                }
                TokenType::SemiColon => Ok(None),
                _ => Err(UnexpectedToken(t.clone(),None))
            }
        }
    }

    fn try_parse_field(&mut self, identifier: &str) -> Result<Option<FieldDeclaration>, ParseError> {
        let scalar_type : ScalarType =  match ScalarType::try_parse(identifier) {
            None => None,
            Some(scalarType) => Some(scalarType)
        }.ok_or(ParseError::UnknownType)?;


        let field_type : FieldType_ = match self.tokens.next() {
            None => Err(ParseError::ExpectedAToken),
            Some(tok) => match tok.get_type() {
                TokenType::OpenBracket => {
                    let r = Ok(FieldType_::Vector(self.try_parse_vector_type(scalar_type)?));
                    self.assert_next_token_matches(TokenType::Colon)?;
                    r
                },
                TokenType::Colon => Ok(FieldType_::Scalar(scalar_type)),
                _ => Err(UnexpectedToken(tok.clone(), None))
            }
        }?;

        let name = self.get_identifier()?.ok_or_else(||ParseError::ExpectedAToken)?;

        self.assert_next_token_matches(TokenType::SemiColon)?;

        Ok(Some(FieldDeclaration{
            name,
            field_type,
            description: None
        }))
    }


    pub fn try_parse_vector_type(&mut self, scalar_type: ScalarType) -> Result<ArrayLike,ParseError>{
        let size = self.get_next_token_if(|t| match t.get_type() {
            TokenType::IntegerLiteral(size) if *size > 0 => Some(size.clone()),
            TokenType::IntegerLiteral(_)  => None,
            _ => None
        }).map_err(|t|match t {
            None => ParseError::ExpectedAToken,
            Some(t) => UnexpectedToken(t.clone(), Some(String::from("Expected an non-zero integer size")))
        })?.clone();

        self.assert_next_token_matches(TokenType::CloseBracket)?;

        if let ScalarType::ByteSized(b) = scalar_type {
            return match b {
                ByteSize::Byte => Ok(ArrayLike::Bytes {length: size}),
                ByteSize::Char => Ok(ArrayLike::AsciiString {length: size}),
                ByteSize::Ascii => Ok(ArrayLike::AsciiString {length: size})
            }
        }

        return Ok(ArrayLike::FixedArray {length: size, scalar: scalar_type})
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

    fn get_next_token_if<F: FnOnce(&Token) -> Option<R>, R>(
        &mut self,
        token_matches: F,
    ) -> Result<R, Option<Token>> {
        match self.tokens.next() {
            None => Err(None),
            Some(token) => match token_matches(&token) {
                None =>  Err(Some(token)),
                Some(v) => Ok(v)
            }
        }
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


#[cfg(test)]
mod test{
    use crate::lexer::tokenize;
    use crate::parser::Parser;

    #[test]
    fn test_parse_message() {
        let mut tokens = tokenize("message foop [123] {\
            12;\
            u32: skibbidy;\
            byte: pow;\
            ascii[10]: txt;\
        \
        }");
        let mut parser = Parser::new(&mut tokens);

        let dec = parser.parse_declaration();


        println!("{:?}",dec)
        //assert!(dec.ok().is_some())
    }
}