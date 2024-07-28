use std::{char, usize};

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Eof,
    Number,
    Operator,
    Invalid,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Tokenizer {
    src: String,
    cursor: usize,
}

impl Tokenizer {
    pub fn new(src: String) -> Self {
        Tokenizer { cursor: 0, src }
    }

    fn is_eof(self: &Self) -> bool {
        return self.cursor >= self.src.len();
    }

    pub fn next(self: &mut Self) -> Token {
        if self.is_eof() {
            return Token {
                token_type: TokenType::Eof,
                value: String::from("EOF"),
            };
        }

        if self.is_empty() {
            self.cursor += 1;
            return self.next();
        }

        if let Some(operator) = self.get_operator() {
            self.cursor += 1;

            return Token {
                token_type: TokenType::Operator,
                value: operator.to_string(),
            };
        }

        if let Some(num) = self.get_number() {
            self.cursor += num.len();

            return Token {
                token_type: TokenType::Number,
                value: num,
            };
        }

        return Token {
            token_type: TokenType::Invalid,
            value: format!(
                "Invalid character `{}`",
                self.src
                    .chars()
                    .nth(self.cursor as usize)
                    .unwrap_or_default(),
            ),
        };
    }

    fn get_operator(self: &Self) -> Option<char> {
        if let Some(char) = self.src.chars().nth(self.cursor as usize) {
            if char == '+' || char == '-' || char == '*' || char == '/' {
                return Some(char);
            }
        }

        None
    }

    fn get_number(self: &Self) -> Option<String> {
        let start = self.cursor as usize;
        let chars = self.src[start..].chars();

        let mut str = String::new();

        for c in chars {
            if c.is_numeric() || c == '.' {
                str.push(c)
            } else {
                break;
            }
        }

        if str == "" {
            None
        } else {
            Some(str)
        }
    }

    fn is_empty(self: &Self) -> bool {
        let char = self.src.chars().nth(self.cursor as usize);

        return match char {
            Some(char) => char == ' ' || char == '\n' || char == '\t',
            None => false,
        };
    }
}
