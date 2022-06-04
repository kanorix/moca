use core::fmt;

use crate::token::Token;

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i32),
    PrefixExpression {
        token: Token,
        operator: String,
        right: Box<Expression>,
    },
    InfixExpression {
        token: Token,
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    Null,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{}", s)?,
            Expression::IntegerLiteral(i) => write!(f, "{}", i)?,
            Expression::PrefixExpression {
                operator, right, ..
            } => write!(f, "({}{})", operator, right)?,
            Expression::InfixExpression {
                left,
                operator,
                right,
                ..
            } => write!(f, "({} {} {})", left, operator, right)?,
            Expression::Null => write!(f, "null")?,
        };
        Ok(())
    }
}
