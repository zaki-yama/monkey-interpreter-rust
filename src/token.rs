#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Illegal,
    Eof,
    // 識別子 + リテラル
    Ident(String), // add, foobar, x, y, ...
    Int(i64),      // 123456

    // 演算子
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,

    // デリミタ
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // キーワード
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
