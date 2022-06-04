use core::fmt;

use crate::ast::statement::Statement;
use crate::token::Token;

#[derive(Debug)]
// Expression（式）は値を生成する
pub enum Expression {
    Identifier(String),
    IntegerLiteral(i32),
    BooleanLiteral(bool),
    FunctionLiteral {
        parameters: Vec<Box<Expression>>, // Identifier
        body: Box<Statement>,             // BlockStatement
    },
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
    IfExpression {
        condition: Box<Expression>,
        consequence: Box<Statement>,
        alternative: Option<Box<Statement>>,
    },
    CallExpression {
        function: Box<Expression>, // Identifier or FunctionLiteral
        arguments: Vec<Box<Expression>>,
    },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(s) => write!(f, "{}", s)?,
            Expression::IntegerLiteral(i) => write!(f, "{}", i)?,
            Expression::BooleanLiteral(b) => write!(f, "{}", b)?,
            Expression::FunctionLiteral { parameters, body } => {
                write!(
                    f,
                    "fn ({}) {}",
                    parameters
                        .iter()
                        .map(|p| format!("{}", p))
                        .collect::<Vec<String>>()
                        .join(", "),
                    body
                )?;
            }
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
            Expression::CallExpression {
                function,
                arguments,
            } => {
                write!(
                    f,
                    "{}({})",
                    function,
                    arguments
                        .iter()
                        .map(|p| format!("{}", p))
                        .collect::<Vec<String>>()
                        .join(", "),
                )?;
            }
        };
        Ok(())
    }
}
