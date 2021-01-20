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
    Return, // TODO: Return(Expression) にする(戻り値の式を保持する)。2.5 時点ではスキップ
}

pub struct Program {
    pub statements: Vec<Statement>,
}
