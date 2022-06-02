use std::{collections::VecDeque, str, vec};
use crate::token::{Token, TokenKind, Value};

pub struct LexicalAnalizer {
    chars: VecDeque<char>,
    current: Option<char>,
    next: Option<char>,
}

impl LexicalAnalizer {
    pub fn new(src: &str) -> Self {
        let mut chars: VecDeque<char> = src.chars().collect();
        let current = chars.pop_front();
        let next = chars.pop_front();
        LexicalAnalizer {
            chars,
            current,
            next,
        }
    }
    fn read(&mut self) {
        self.current = self.next;
        self.next = self.chars.pop_front();
        // println!("read: [{:?}, {:?}]", self.current, self.next);
    }

    fn skip_space(&mut self) {
        match self.current {
            Some(c) => {
                if c.is_whitespace() {
                    self.read();
                    self.skip_space();
                }
            }
            None => (),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_space();

        self.current.and_then(|c| {
            if c.is_numeric() {
                Some(self.find_numeric())
            } else if Self::is_symbol(c) {
                Some(self.find_symbol())
            } else if c.is_alphabetic() {
                Some(self.find_word())
            } else if c == '"' {
                Some(self.find_string())
            } else {
                self.read();
                Some(Token::new(TokenKind::Other, c.to_string(), Value::None))
            }
        })
    }

    fn is_symbol(c: char) -> bool {
        vec!['=', '+', '-', '*', '/', '!', '.', '&', '|', ';','(', ')', '{', '}', '[', ']'].contains(&c)
    }

    fn is_end(c: char) -> bool {
        vec!['\n', '\r', ';'].contains(&c)
    }

    fn find_symbol(&mut self) -> Token {

        let op = self.current.unwrap();
        let mut text = op.to_string();
        let kind = match op {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multi,
            '/' => TokenKind::Div,
            '.' => TokenKind::Dot,
            '(' => TokenKind::Lparen,
            ')' => TokenKind::Rparen,
            '{' => TokenKind::Lcurly,
            '}' => TokenKind::Rcurly,
            '[' => TokenKind::Lsquare,
            ']' => TokenKind::Rsquare,
            ';' => TokenKind::Semicolon,
            '=' => {
                match self.next {
                    Some('=') => {
                        self.read();
                        text.push('=');
                        TokenKind::Equal
                    },
                    _ => TokenKind::Assign,
                }
            },
            '!' => {
                match self.next {
                    Some('=') => {
                        self.read();
                        text.push('=');
                        TokenKind::NotEqual
                    },
                    _ => TokenKind::Not,
                }
            },
            '&' => {
                match self.next {
                    Some('&') => {
                        self.read();
                        text.push('&');
                        TokenKind::And
                    },
                    _ => TokenKind::Other,
                }
            },
            '|' => {
                match self.next {
                    Some('|') => {
                        self.read();
                        text.push('|');
                        TokenKind::Or
                    },
                    _ => TokenKind::Other,
                }
            },
            _ => TokenKind::Other,
        };
        self.read();
        Token::new(kind, text, Value::None)
    }

    fn find_word(&mut self) -> Token {
        let mut stack = Vec::new();

        while let Some(c) = self.current {
            if c.is_whitespace() || Self::is_symbol(c) || Self::is_end(c) {
                break;
            } else {
                stack.push(c);
                self.read();
            }
        }

        let text: String = stack.iter().collect();
        match text.as_str() {
            "if" => Token::new(TokenKind::If, text, Value::None),
            "else" => Token::new(TokenKind::Else, text, Value::None),
            "new" => Token::new(TokenKind::New, text, Value::None),
            "true" => Token::new(TokenKind::Literal, text, Value::Boolean(true)),
            "false" => Token::new(TokenKind::Literal, text, Value::Boolean(false)),
            "void" => Token::new(TokenKind::Literal, text, Value::Void),
            "class" => Token::new(TokenKind::Class, text, Value::None),
            _ => Token::new(TokenKind::Ident, text, Value::None),
        }
    }

    fn find_string(&mut self) -> Token {
        let mut stack = Vec::new();

        // 始まりの " を飛ばす
        self.read();
        while let Some(s) = self.current {
            if s != '"' {
                // 終わりの " がくるまでpushし続ける
                stack.push(s);
                self.read();
            } else {
                // 終わりの " を飛ばす
                self.read();
                break;
            }
        }
        let value: String = stack.iter().collect();
        Token::new(TokenKind::Literal, value.clone(), Value::String(value))
    }

    fn find_numeric(&mut self) -> Token {
        let mut stack = Vec::new();
        let mut dot_flg = false;

        while let Some(s) = self.current {
            if s.is_numeric() {
                stack.push(s);
                self.read();
            } else if s == '.' && !dot_flg {
                dot_flg = true;
                stack.push(s);
                self.read();
            } else {
                break;
            }
        }
        let numeric: String = stack.iter().collect();

        let value = if dot_flg {
            Value::Double(numeric.parse().unwrap())
        } else {
            Value::Int(numeric.parse().unwrap())
        };
        Token::new(TokenKind::Literal, numeric, value)
    }
}
