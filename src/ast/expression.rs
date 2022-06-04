use core::fmt;

use crate::ast::statement::Statement;
use crate::token::Token;

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i32),
    BooleanLiteral(bool),
    // <operator> <rignt>
    PrefixExpression {
        token: Token,
        operator: String,
        right: Box<Expression>,
    },
    // <left> <operator> <right>
    InfixExpression {
        token: Token,
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    // if (<condition>) <consequence> else <alternative>
    IfExpression {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    Null,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{}", s)?,
            Expression::IntegerLiteral(i) => write!(f, "{}", i)?,
            Expression::BooleanLiteral(b) => write!(f, "{}", b)?,
            Expression::PrefixExpression {
                operator, right, ..
            } => write!(f, "({}{})", operator, right)?,
            Expression::InfixExpression {
                left,
                operator,
                right,
                ..
            } => write!(f, "({} {} {})", left, operator, right)?,
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
                ..
            } => match alternative {
                Some(alt) => write!(
                    f,
                    "if ({}) \n\t{} \nelse\n\t {})",
                    condition, consequence, alt
                )?,
                _ => write!(f, "if ({}) \n\t{} ", condition, consequence)?,
            },
            Expression::Null => write!(f, "null")?,
        };
        Ok(())
    }
}
