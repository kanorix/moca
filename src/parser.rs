use crate::ast::expression::Expression;
use crate::ast::program::Program;
use crate::ast::statement::Statement;
use crate::lexer::Lexer;
use crate::token::{get_priority, Priority, Token, TokenKind};
use core::fmt;

pub struct Parser {
    lexer: Lexer,
    token: Token,
    peek: Option<Token>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let peek = lexer.next_token();
        Parser {
            lexer,
            token: Token::new(TokenKind::Other, "".to_string()),
            peek,
        }
    }

    fn expect_next(&mut self, token_kind: TokenKind) -> Result<Token, ParseError> {
        match &self.peek {
            Some(t) if t.is_same_kind(token_kind) => {
                self.next_token();
                Result::Ok(self.token.clone())
            }
            Some(t) => {
                let message = format!(
                    "expected next token to be {:?}, got {:?} instead",
                    token_kind, t.token_kind
                );
                ParseError::throw(message)
            }
            _ => {
                let message = format!("expected next token is {:?}", token_kind);
                ParseError::throw(message)
            }
        }
    }

    fn peek_token_is(&self, token_kind: TokenKind) -> bool {
        match &self.peek {
            Some(p) => p.is_same_kind(token_kind),
            None => false,
        }
    }

    fn next_token(&mut self) -> bool {
        match &self.peek {
            Some(p) => {
                self.token = p.clone();
                self.peek = self.lexer.next_token();
                true
            }
            None => false,
        }
    }

    fn skip_to(&mut self, token_kind: TokenKind) {
        while !self.token.is_same_kind(token_kind) && self.next_token() {}
    }

    fn parse_program(&mut self) -> Program {
        let program = Program::new();

        while self.next_token() {
            match self.parse_statement() {
                Ok(stmt) => println!("{}", stmt),
                Err(err) => err.println(),
            }
        }
        return program;
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.token.token_kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.token.clone(); // let のはず
        let ident = self.expect_next(TokenKind::Ident)?;
        self.expect_next(TokenKind::Assign)?;

        self.skip_to(TokenKind::SemiColon);

        Result::Ok(Statement::LetStatement {
            token,
            identfier: Expression::Identifier(ident.value),
            value: Expression::Null,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.token.clone(); // return のはず

        self.skip_to(TokenKind::SemiColon);

        Result::Ok(Statement::ReturnStatement {
            token,
            return_value: Expression::Null,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.token.clone();

        let expression = self.parse_expression(Priority::Lowest as u8)?;

        if self.peek_token_is(TokenKind::SemiColon) {
            self.next_token();
        }

        Result::Ok(Statement::ExpressionStatement { token, expression })
    }

    fn parse_expression(&mut self, priority: u8) -> Result<Expression, ParseError> {
        // prefix
        let mut left = match self.token.token_kind {
            TokenKind::Ident => self.parse_identifier()?,
            TokenKind::IntLiteral => self.parse_integer_literal()?,
            TokenKind::Bang | TokenKind::Minus => self.parse_prefix_expression()?,
            _ => ParseError::throw("no prefix".to_string())?,
        };

        while !self.peek_token_is(TokenKind::SemiColon) && priority < self.peek_priority() as u8 {
            let peek = self.peek.clone().unwrap();
            let right = match peek.token_kind {
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Slash
                | TokenKind::Asterisk
                | TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::GreaterThan => Self::parse_infix_expression,
                _ => break,
            };
            self.next_token();
            left = right(self, left)?;
        }
        return Result::Ok(left);
    }

    fn peek_priority(&self) -> Priority {
        match &self.peek {
            Some(peek) => get_priority(peek.token_kind),
            None => Priority::Lowest,
        }
    }

    fn parse_identifier(&self) -> Result<Expression, ParseError> {
        Result::Ok(Expression::Identifier(self.token.clone().value))
    }

    fn parse_integer_literal(&self) -> Result<Expression, ParseError> {
        match self.token.value.parse::<i32>() {
            Ok(number) => Result::Ok(Expression::IntegerLiteral(number)),
            Err(_) => {
                ParseError::throw(format!("could not parse {} as integer.", self.token.value))
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Result<Expression, ParseError> {
        let token = self.token.clone();
        let operator = token.value.clone();
        self.next_token();
        let right = Box::new(self.parse_expression(Priority::Prefix as u8)?);
        Ok(Expression::PrefixExpression {
            token,
            operator,
            right,
        })
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Result<Expression, ParseError> {
        let token = self.token.clone();
        let operator = token.value.clone();
        let priority = get_priority(self.token.token_kind) as u8;
        self.next_token();

        Ok(Expression::InfixExpression {
            token,
            left: Box::new(left),
            operator,
            right: Box::new(self.parse_expression(priority)?),
        })
    }
}

mod tests {
    pub use crate::lexer::Lexer;
    pub use crate::parser::Parser;

    #[test]
    fn next_token() {
        let src = r#"
        a + b * c;
        a * b * c;
        "#;
        let mut pa = Parser::new(Lexer::new(src));
        pa.parse_program();
    }
}

struct ParseError {
    message: String,
    uncheck: bool,
}

impl ParseError {
    fn throw<T>(message: String) -> Result<T, Self> {
        Result::Err(ParseError {
            message,
            uncheck: false,
        })
    }

    fn uncheck<T>() -> Result<T, Self> {
        Result::Err(ParseError {
            message: "".to_string(),
            uncheck: true,
        })
    }

    fn println(&self) {
        if !self.uncheck {
            println!("{}", self);
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: {}", self.message)?;
        Ok(())
    }
}
