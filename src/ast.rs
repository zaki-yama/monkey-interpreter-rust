use std::fmt;
pub enum Node {
    Statement,
    Expression,
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(String),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(value) => write!(f, "{}", value),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let { name: Expression },
    Return, // TODO: Return(Expression) にする(戻り値の式を保持する)。2.5 時点ではスキップ
    Expression(Expression),
}

pub struct Program {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for statement in self.statements.iter() {
            write!(f, "{}\n", statement)?;
        }
        Ok(())
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let { name } => write!(f, "let {} = ", name),
            Statement::Return => write!(f, "return (TODO: 式);"),
            Statement::Expression(expression) => write!(f, "{}", expression),
        }
    }
}
