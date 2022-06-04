use core::fmt;

use super::expression::Expression;
use crate::token::Token;

#[derive(Debug)]
pub enum Statement {
    LetStatement {
        token: Token,
        identfier: Expression,
        value: Expression,
    },
    ReturnStatement {
        token: Token,
        return_value: Expression,
    },
    ExpressionStatement {
        token: Token,
        expression: Expression,
    },
    BlockStatement {
        token: Token,
        statements: Vec<Statement>,
    },
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::LetStatement {
                identfier, value, ..
            } => write!(f, "let {} = {};", identfier, value)?,
            Statement::ReturnStatement { return_value, .. } => {
                write!(f, "return {};", return_value)?
            }
            Statement::ExpressionStatement { expression, .. } => write!(f, "{}", expression)?,
            Statement::BlockStatement { statements, .. } => {
                write!(f, "{{\n")?;
                for stmt in statements {
                    write!(f, "{}", stmt)?;
                }
                write!(f, "\n}}")?;
            }
        };
        Ok(())
    }
}
