pub mod token;
pub mod lexer;

use crate::token::Token;

#[test]
fn test_next_token() {
    let token = Token::Comma;
    assert_eq!(Token::Comma, token);
}
