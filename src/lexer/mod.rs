use crate::source::{Point, Range};
use crate::token;
pub use token::{Token, TokenType};

mod token;

#[derive(Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    curr_offset: usize,
    current_location: Point,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> + Clone + '_ {
    Lexer::new(input)
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
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
            current_location: Point::zero(),
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
                Some(_) => {
                    break;
                }
                None => return None,
            }
        }
        let first_char = self.current_char()?;
        let start_loc = self.current_location.clone();

        const KEY_CHARS: [char; 8] = ['[', ']', '{', '}', ',', ':', ';', '.'];

        return match first_char {
            char if char.is_ascii_alphabetic() => {
                let identifier: String = self
                    .input
                    .chars()
                    .skip(self.curr_offset)
                    .take_while(move |c| is_identifier_char(*c))
                    .collect();

                self.advance_cursor(identifier.len());
                Some(token!(
                        Identifier,
                        Range::new(start_loc, self.current_location),
                        identifier
                    )
                )
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
                    Range::new(start_loc, self.current_location),
                    num
                ));
            }
            char if KEY_CHARS.contains(&char) => {
                _ = self.next_char();
                return match char {
                    '[' => Some(token!(OpenBracket, start_loc)),
                    ']' => Some(token!(CloseBracket, start_loc)),
                    '{' => Some(token!(OpenBrace, start_loc)),
                    '}' => Some(token!(CloseBrace, start_loc)),
                    ',' => Some(token!(Comma, start_loc)),
                    ':' => Some(token!(Colon, start_loc)),
                    '.' => Some(token!(Dot, start_loc)),
                    ';' => Some(token!(SemiColon, start_loc)),
                    _ => panic!("unreachable"),
                };
            }
            _ => None,
        };
    }
}
