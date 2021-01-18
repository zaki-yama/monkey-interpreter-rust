pub enum Node {
    Statement,
    Expression,
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Identifier(String),
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let { name: Expression },
}

pub struct Program {
    pub statements: Vec<Statement>,
}
