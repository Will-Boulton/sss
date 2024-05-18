use crate::lexer::token::lex_keyword;
use crate::source::{SourceLocation, SourcePoint, ToLocation};
use crate::{location, token};
pub use token::{Keyword, Token, TokenType};

mod token;

#[derive(Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    curr_offset: usize,
    current_location: SourcePoint,
    line_pos: usize,
    line: usize,
    ch: char,
    prev_token: Option<TokenType>,
}
const EOF_CHAR: char = '\0';

pub fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Token> + 'a {
    Lexer::new(input)
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

pub(crate) enum LexerError {
    InvalidInteger(SourcePoint),
    InvalidIdentifier(SourcePoint),
    UnexpectedError,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'a> Lexer<'a> {
    pub(self) fn new(input: &'a str) -> Self {
        Self {
            input,
            curr_offset: 0,
            current_location: SourcePoint::zero(),
            line_pos: 0,
            line: 0,
            ch: EOF_CHAR,
            prev_token: None,
        }
    }

    fn advance_cursor(&mut self, by: usize) {
        for _ in 0..by {
            if self.next_char() == None {
                break;
            }
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.curr_offset)
    }

    fn has_current_char(&self) -> bool {
        self.current_char().is_some()
    }

    fn has_next_char(&self) -> bool {
        self.peek_next_char().is_some()
    }

    fn peek_next_char(&self) -> Option<char> {
        let next_offset = self.curr_offset + 1;
        self.input.chars().nth(next_offset)
    }

    fn next_char(&mut self) -> Option<char> {
        self.curr_offset += 1;
        if let Some(x) = self.peek_next_char() {
            self.current_location = self.current_location.next_pos();
            if x == '\n' {
                self.current_location = self.current_location.next_line();
            }
            return Some(x);
        }
        None
    }

    fn next_token(&mut self) -> Option<Token> {
        if !self.has_current_char() {
            return None;
        }
        loop {
            match self.current_char() {
                Some(ch) if ch.is_whitespace() => {
                    self.next_char();
                }
                Some(ch) => {
                    break;
                }
                None => return None,
            }
        }
        let first_char = self.current_char()?;
        let start_loc = self.current_location.clone();

        const KEY_CHARS: [char; 7] = ['[', ']', '{', '}', ',', ':', ';'];

        return match first_char {
            char if char.is_ascii_alphabetic() => {
                let identifier: String = self
                    .input
                    .chars()
                    .skip(self.curr_offset)
                    .take_while(move |c| is_identifier_char(*c))
                    .collect();

                self.advance_cursor(identifier.len());
                return match lex_keyword(identifier.as_str()) {
                    Some(keyword) => Some(token!(
                        Keyword,
                        location!(start_loc => self.current_location),
                        keyword
                    )),
                    _ => Some(token!(
                        Identifier,
                        location!(start_loc => self.current_location),
                        identifier
                    )),
                };
            }
            char if char.is_numeric() => {
                let num: String = self
                    .input
                    .chars()
                    .skip(self.curr_offset)
                    .take_while(move |c| c.is_numeric())
                    .collect();
                self.advance_cursor(num.len());
                return Some(token!(
                    IntegerLiteral,
                    location!(start_loc => self.current_location),
                    num
                ));
            }
            char if KEY_CHARS.contains(&char) => {
                _ = self.next_char();
                return match char {
                    '[' => Some(token!(OpenBracket, start_loc.to_location())),
                    ']' => Some(token!(CloseBracket, start_loc.to_location())),
                    '{' => Some(token!(OpenBrace, start_loc.to_location())),
                    '}' => Some(token!(CloseBrace, start_loc.to_location())),
                    ',' => Some(token!(Comma, start_loc.to_location())),
                    ':' => Some(token!(Colon, start_loc.to_location())),
                    ';' => Some(token!(SemiColon, start_loc.to_location())),
                    _ => panic!("unreachable"),
                };
            }
            _ => None,
        };
    }
}
