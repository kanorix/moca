use crate::token::{Token, TokenKind, Value};
use std::{collections::VecDeque, str, vec};

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

    fn skip_while(&mut self, test: impl Fn(char) -> bool) {
        while let Some(c) = self.current {
            if test(c) {
                self.read()
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_while(char::is_whitespace);
        self.skip_comment();
        self.skip_while(char::is_whitespace);

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
                Some(Token::new(TokenKind::Other, Value::Symbol(c.to_string())))
            }
        })
    }

    fn is_symbol(c: char) -> bool {
        vec![
            '=', '+', '-', '*', '/', '!', '.', '&', '|', ';', '(', ')', '{', '}', '[', ']', ':',
        ]
        .contains(&c)
    }

    fn is_end(c: char) -> bool {
        vec!['\n', '\r', '\0'].contains(&c)
    }

    fn skip_comment(&mut self) {
        if Some('/') == self.current {
            match self.next {
                Some('/') => {
                    self.skip_while(|c| !Self::is_end(c));
                }
                _ => (),
            }
        }
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
            ';' => TokenKind::Scolon,
            ':' => TokenKind::Colon,
            '=' => match self.next {
                Some('=') => {
                    self.read();
                    text.push('=');
                    TokenKind::Equal
                }
                _ => TokenKind::Assign,
            },
            '!' => match self.next {
                Some('=') => {
                    self.read();
                    text.push('=');
                    TokenKind::NotEqual
                }
                _ => TokenKind::Not,
            },
            '&' => match self.next {
                Some('&') => {
                    self.read();
                    text.push('&');
                    TokenKind::And
                }
                _ => TokenKind::Other,
            },
            '|' => match self.next {
                Some('|') => {
                    self.read();
                    text.push('|');
                    TokenKind::Or
                }
                _ => TokenKind::Other,
            },
            _ => TokenKind::Other,
        };
        self.read();
        Token::new(kind, Value::Symbol(text))
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
            "if" => Token::new(TokenKind::If, Value::Symbol(text)),
            "else" => Token::new(TokenKind::Else, Value::Symbol(text)),
            "for" => Token::new(TokenKind::For, Value::Symbol(text)),
            "new" => Token::new(TokenKind::New, Value::Symbol(text)),
            "void" => Token::new(TokenKind::Void, Value::Symbol(text)),
            "class" => Token::new(TokenKind::Class, Value::Symbol(text)),
            "true" => Token::new(TokenKind::Literal, Value::Boolean(true)),
            "false" => Token::new(TokenKind::Literal, Value::Boolean(false)),
            "int" => Token::new(TokenKind::Int, Value::Symbol(text)),
            "double" => Token::new(TokenKind::Double, Value::Symbol(text)),
            "boolean" => Token::new(TokenKind::Boolean, Value::Symbol(text)),
            _ => Token::new(TokenKind::Ident, Value::Symbol(text)),
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
        Token::new(TokenKind::Literal, Value::String(value))
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
            Value::Integer(numeric.parse().unwrap())
        };
        Token::new(TokenKind::Literal, value)
    }
}

#[cfg(test)]
mod tests {
    use super::LexicalAnalizer;

    #[test]
    fn next_token() {
        let src = "
        int abc = 1234; //this is comment
        boolean b = true;
        ";
        let mut la = LexicalAnalizer::new(src);
        while let Some(token) = la.next_token() {
            println!("{}", token.to_string());
        }
    }
}
