use crate::token::{Token, TokenKind};
use std::{collections::VecDeque, str, vec};

pub struct Lexer {
    chars: VecDeque<char>,
    current: Option<char>,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        let mut chars: VecDeque<char> = src.chars().collect();
        let current = chars.pop_front();
        Lexer { chars, current }
    }

    fn read_char(&mut self) {
        self.current = self.chars.pop_front();
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.front().map(&char::to_owned)
    }

    fn skip_while(&mut self, test: impl Fn(char) -> bool) {
        while let Some(c) = self.current {
            if test(c) {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn get_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut chars = Vec::new();
        while let Some(c) = self.current {
            if test(c) {
                chars.push(c);
                self.read_char()
            } else {
                break;
            }
        }
        chars.iter().collect()
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
                self.read_char();
                Some(Token::new(TokenKind::Other, c.to_string()))
            }
        })
    }

    fn is_symbol(c: char) -> bool {
        vec![
            '=', '+', '-', '*', '/', '!', '&', '|', ';', '(', ')', '{', '}', '[', ']', ':',
        ]
        .contains(&c)
    }

    fn is_end(c: char) -> bool {
        vec!['\n', '\r', '\0'].contains(&c)
    }

    fn skip_comment(&mut self) {
        if Some('/') == self.current {
            match self.peek_char() {
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
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::Lparen,
            ')' => TokenKind::Rparen,
            '{' => TokenKind::Lcurly,
            '}' => TokenKind::Rcurly,
            '[' => TokenKind::Lsquare,
            ']' => TokenKind::Rsquare,
            ';' => TokenKind::SemiColon,
            ':' => TokenKind::Colon,
            '>' => TokenKind::LessThan,
            '<' => TokenKind::GreaterThan,
            '=' => match self.peek_char() {
                Some('=') => {
                    self.read_char();
                    text.push('=');
                    TokenKind::Equal
                }
                _ => TokenKind::Assign,
            },
            '!' => match self.peek_char() {
                Some('=') => {
                    self.read_char();
                    text.push('=');
                    TokenKind::NotEqual
                }
                _ => TokenKind::Bang,
            },
            '&' => match self.peek_char() {
                Some('&') => {
                    self.read_char();
                    text.push('&');
                    TokenKind::And
                }
                _ => TokenKind::Other,
            },
            '|' => match self.peek_char() {
                Some('|') => {
                    self.read_char();
                    text.push('|');
                    TokenKind::Or
                }
                _ => TokenKind::Other,
            },
            _ => TokenKind::Other,
        };
        self.read_char();
        Token::new(kind, text)
    }

    fn find_word(&mut self) -> Token {
        let value: String =
            self.get_while(|c| !(c.is_whitespace() || Self::is_symbol(c) || Self::is_end(c)));
        match value.as_str() {
            "void" => Token::new(TokenKind::Void, value),
            "return" => Token::new(TokenKind::Return, value),
            "if" => Token::new(TokenKind::If, value),
            "else" => Token::new(TokenKind::Else, value),
            "let" => Token::new(TokenKind::Let, value),
            "while" => Token::new(TokenKind::While, value),
            "true" => Token::new(TokenKind::BoolLiteral, value),
            "false" => Token::new(TokenKind::BoolLiteral, value),
            "int" => Token::new(TokenKind::Int, value),
            "double" => Token::new(TokenKind::Double, value),
            "boolean" => Token::new(TokenKind::Boolean, value),
            _ => Token::new(TokenKind::Ident, value),
        }
    }

    fn find_string(&mut self) -> Token {
        // 始まりの " を飛ばす
        self.read_char();

        let value: String = self.get_while(|c| c != '"');

        // 終わりの " を飛ばす
        self.read_char();

        Token::new(TokenKind::StringLiteral, value)
    }

    fn find_numeric(&mut self) -> Token {
        let mut stack = Vec::new();
        let mut dot_flg = false;

        while let Some(s) = self.current {
            if s.is_numeric() {
                stack.push(s);
                self.read_char();
            } else if s == '.' && !dot_flg {
                dot_flg = true;
                stack.push(s);
                self.read_char();
            } else {
                break;
            }
        }

        let numeric: String = stack.iter().collect();
        if dot_flg {
            Token::new(TokenKind::DoubleLiteral, numeric)
        } else {
            Token::new(TokenKind::IntLiteral, numeric)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;

    #[test]
    fn next_token() {
        let src = r#"
        int abc = 1234; //this is comment
        boolean b = true;
        String str = "sss;";
        "#;
        let mut la = Lexer::new(src);
        while let Some(token) = la.next_token() {
            println!("{}", token.to_string());
        }
    }
}
