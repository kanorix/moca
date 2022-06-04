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

    pub fn parse_program(&mut self) -> Program {
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

        self.next_token();
        let value = self.parse_expression(Priority::Lowest as u8)?;

        if self.peek_token_is(TokenKind::SemiColon) {
            self.next_token();
        }

        Result::Ok(Statement::LetStatement {
            token,
            identfier: Expression::Identifier(ident.value),
            value,
        })
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.token.clone(); // return のはず

        self.next_token();
        let return_value = self.parse_expression(Priority::Lowest as u8)?;

        if self.peek_token_is(TokenKind::SemiColon) {
            self.next_token();
        }

        Ok(Statement::ReturnStatement {
            token,
            return_value,
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.token.clone();

        let expression = self.parse_expression(Priority::Lowest as u8)?;

        if self.peek_token_is(TokenKind::SemiColon) {
            self.next_token();
        }

        Ok(Statement::ExpressionStatement { token, expression })
    }

    fn parse_block_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.token.clone();
        let mut statements: Vec<Statement> = Vec::new();
        self.next_token();
        while !self.token.is_same_kind(TokenKind::Rcurly) {
            statements.push(self.parse_statement()?);
            self.next_token();
        }
        Ok(Statement::BlockStatement { token, statements })
    }

    fn parse_expression(&mut self, priority: u8) -> Result<Expression, ParseError> {
        // prefix
        let mut left = match self.token.token_kind {
            TokenKind::Ident => self.parse_identifier()?,
            TokenKind::IntLiteral => self.parse_integer_literal()?,
            TokenKind::BoolLiteral => self.parse_boolean_literal()?,
            TokenKind::Bang | TokenKind::Minus => self.parse_prefix_expression()?,
            TokenKind::Lparen => self.parse_grouped_expression()?,
            TokenKind::If => self.parse_if_expression()?,
            TokenKind::Fn => self.parse_function_literal()?,
            other => ParseError::throw(format!("no prefix but found {:?}", other))?,
        };

        while !self.peek_token_is(TokenKind::SemiColon) && priority < self.peek_priority() as u8 {
            let peek = self.peek.clone().unwrap();
            // infix
            let right = match peek.token_kind {
                TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Slash
                | TokenKind::Asterisk
                | TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::LessThan
                | TokenKind::GreaterThan => Self::parse_infix_expression,
                TokenKind::Lparen => Self::parse_call_expression,
                _ => break,
            };
            self.next_token();
            left = right(self, left)?;
        }
        return Ok(left);
    }

    fn peek_priority(&self) -> Priority {
        match &self.peek {
            Some(peek) => get_priority(peek.token_kind),
            None => Priority::Lowest,
        }
    }

    fn parse_identifier(&self) -> Result<Expression, ParseError> {
        Ok(Expression::Identifier(self.token.clone().value))
    }

    fn parse_integer_literal(&self) -> Result<Expression, ParseError> {
        match self.token.value.parse::<i32>() {
            Ok(number) => Ok(Expression::IntegerLiteral(number)),
            Err(_) => {
                ParseError::throw(format!("could not parse {} as integer.", self.token.value))
            }
        }
    }

    fn parse_boolean_literal(&self) -> Result<Expression, ParseError> {
        match self.token.value.parse::<bool>() {
            Ok(boolean) => Ok(Expression::BooleanLiteral(boolean)),
            Err(_) => {
                ParseError::throw(format!("could not parse {} as boolean.", self.token.value))
            }
        }
    }

    fn parse_function_literal(&mut self) -> Result<Expression, ParseError> {
        self.expect_next(TokenKind::Lparen)?; // (

        let parameters = self.parse_function_params()?;

        self.expect_next(TokenKind::Lcurly)?; // {

        let body = Box::new(self.parse_block_statement()?);

        Ok(Expression::FunctionLiteral { parameters, body })
    }

    fn parse_function_params(&mut self) -> Result<Vec<Box<Expression>>, ParseError> {
        let mut params = Vec::new();

        self.next_token();
        if self.token.is_same_kind(TokenKind::Rparen) {
            return Ok(params);
        }

        params.push(Box::new(Expression::Identifier(self.token.clone().value)));

        while self.peek_token_is(TokenKind::Comma) {
            self.next_token();
            self.next_token();
            params.push(Box::new(Expression::Identifier(self.token.clone().value)));
        }

        self.expect_next(TokenKind::Rparen)?; // )
        Ok(params)
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

    fn parse_grouped_expression(&mut self) -> Result<Expression, ParseError> {
        self.next_token();

        let expression = self.parse_expression(Priority::Lowest as u8)?;
        self.next_token();

        if self.peek_token_is(TokenKind::Rparen) {
            ParseError::throw("expected ')' but not found.".to_string())?
        }

        Ok(expression)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParseError> {
        self.expect_next(TokenKind::Lparen)?; // (

        self.next_token();
        let condition = self.parse_expression(Priority::Lowest as u8)?;

        self.expect_next(TokenKind::Rparen)?; // )
        self.expect_next(TokenKind::Lcurly)?; // {

        let consequence = Box::new(self.parse_block_statement()?);

        let alternative = if self.peek_token_is(TokenKind::Else) {
            self.next_token();
            self.expect_next(TokenKind::Lcurly)?;
            Some(Box::new(self.parse_block_statement()?))
        } else {
            None
        };

        Ok(Expression::IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        })
    }

    fn parse_call_expression(&mut self, function: Expression) -> Result<Expression, ParseError> {
        Ok(Expression::CallExpression {
            function: Box::new(function),
            arguments: self.parse_call_arguments()?,
        })
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Box<Expression>>, ParseError> {
        let mut arguments = Vec::new();

        self.next_token();
        if self.token.is_same_kind(TokenKind::Rparen) {
            return Ok(arguments);
        }

        arguments.push(Box::new(self.parse_expression(Priority::Lowest as u8)?));

        while self.peek_token_is(TokenKind::Comma) {
            self.next_token();
            self.next_token();
            arguments.push(Box::new(self.parse_expression(Priority::Lowest as u8)?));
        }

        self.expect_next(TokenKind::Rparen)?; // )
        Ok(arguments)
    }
}

mod tests {
    pub use crate::lexer::Lexer;
    pub use crate::parser::Parser;

    #[test]
    fn next_token() {
        let src = r#"
        let a = 10;
        let b;
        if (true) {
            let a = 10;
        };
        if (true) {
            let a = 10; a + 10;
        } else {
            let b = 100;
        }
        "#;
        let src = r#"
        let add = fn (a,b) {
            return a + b;
        }
        "#;
        let src = r#"
        let add = fn () {
            return a + b;
        }
        "#;
        let src = r#"
        add(a, b);
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
        Err(ParseError {
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
