#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    Illegal,
    Eof,
    // 識別子 + リテラル
    Ident, // add, foobar, x, y, ...
    Int,   // 123456

    // 演算子
    Assign,
    Plus,

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
}
