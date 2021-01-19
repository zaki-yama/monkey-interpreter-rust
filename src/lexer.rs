use crate::token::{lookup_ident, Token, TokenType};
use log::debug;

#[test]
fn test_next_token() {
    env_logger::init();

    let input = String::from(
        "let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;
    ",
    );
    let mut l = Lexer::new(&input);
    let tests = vec![
        Token::new(TokenType::Let, String::from("let")),
        Token::new(TokenType::Ident, String::from("five")),
        Token::new(TokenType::Assign, String::from("=")),
        Token::new(TokenType::Int, String::from("5")),
        Token::new(TokenType::Semicolon, String::from(";")),
        //
        Token::new(TokenType::Let, String::from("let")),
        Token::new(TokenType::Ident, String::from("ten")),
        Token::new(TokenType::Assign, String::from("=")),
        Token::new(TokenType::Int, String::from("10")),
        Token::new(TokenType::Semicolon, String::from(";")),
        //
        Token::new(TokenType::Let, String::from("let")),
        Token::new(TokenType::Ident, String::from("add")),
        Token::new(TokenType::Assign, String::from("=")),
        Token::new(TokenType::Function, String::from("fn")),
        Token::new(TokenType::LParen, String::from("(")),
        Token::new(TokenType::Ident, String::from("x")),
        Token::new(TokenType::Comma, String::from(",")),
        Token::new(TokenType::Ident, String::from("y")),
        Token::new(TokenType::RParen, String::from(")")),
        Token::new(TokenType::LBrace, String::from("{")),
        Token::new(TokenType::Ident, String::from("x")),
        Token::new(TokenType::Plus, String::from("+")),
        Token::new(TokenType::Ident, String::from("y")),
        Token::new(TokenType::Semicolon, String::from(";")),
        Token::new(TokenType::RBrace, String::from("}")),
        Token::new(TokenType::Semicolon, String::from(";")),
        // let result = add(five, ten);
        Token::new(TokenType::Let, String::from("let")),
        Token::new(TokenType::Ident, String::from("result")),
        Token::new(TokenType::Assign, String::from("=")),
        Token::new(TokenType::Ident, String::from("add")),
        Token::new(TokenType::LParen, String::from("(")),
        Token::new(TokenType::Ident, String::from("five")),
        Token::new(TokenType::Comma, String::from(",")),
        Token::new(TokenType::Ident, String::from("ten")),
        Token::new(TokenType::RParen, String::from(")")),
        Token::new(TokenType::Semicolon, String::from(";")),
        // !-/*5;
        Token::new(TokenType::Bang, String::from("!")),
        Token::new(TokenType::Minus, String::from("-")),
        Token::new(TokenType::Slash, String::from("/")),
        Token::new(TokenType::Asterisk, String::from("*")),
        Token::new(TokenType::Int, String::from("5")),
        Token::new(TokenType::Semicolon, String::from(";")),
        // 5 < 10 > 5;
        Token::new(TokenType::Int, String::from("5")),
        Token::new(TokenType::Lt, String::from("<")),
        Token::new(TokenType::Int, String::from("10")),
        Token::new(TokenType::Gt, String::from(">")),
        Token::new(TokenType::Int, String::from("5")),
        Token::new(TokenType::Semicolon, String::from(";")),
        // if (5 < 10) {
        Token::new(TokenType::If, String::from("if")),
        Token::new(TokenType::LParen, String::from("(")),
        Token::new(TokenType::Int, String::from("5")),
        Token::new(TokenType::Lt, String::from("<")),
        Token::new(TokenType::Int, String::from("10")),
        Token::new(TokenType::RParen, String::from(")")),
        Token::new(TokenType::LBrace, String::from("{")),
        //     return true;
        Token::new(TokenType::Return, String::from("return")),
        Token::new(TokenType::True, String::from("true")),
        Token::new(TokenType::Semicolon, String::from(";")),
        // } else {
        Token::new(TokenType::RBrace, String::from("}")),
        Token::new(TokenType::Else, String::from("else")),
        Token::new(TokenType::LBrace, String::from("{")),
        //     return false;
        Token::new(TokenType::Return, String::from("return")),
        Token::new(TokenType::False, String::from("false")),
        Token::new(TokenType::Semicolon, String::from(";")),
        // }
        Token::new(TokenType::RBrace, String::from("}")),
        // 10 == 10;
        Token::new(TokenType::Int, String::from("10")),
        Token::new(TokenType::Eq, String::from("==")),
        Token::new(TokenType::Int, String::from("10")),
        Token::new(TokenType::Semicolon, String::from(";")),
        // 10 != 9;
        Token::new(TokenType::Int, String::from("10")),
        Token::new(TokenType::NotEq, String::from("!=")),
        Token::new(TokenType::Int, String::from("9")),
        Token::new(TokenType::Semicolon, String::from(";")),
        Token::new(TokenType::Eof, String::from("")),
    ];

    for expected in tests {
        let token = l.next_token();
        debug!("token: {:?}", token);
        assert_eq!(expected, token);
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,      // 入力における現在の位置(現在の文字を指し示す)
    read_position: usize, // これから読み込む位置(現在の文字の次)
    ch: u8,               // 現在検査中の文字
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();

        return lexer;
    }

    fn read_char(&mut self) {
        debug!(
            "(read_position, ch): ({}, {})",
            self.read_position, self.ch as char
        );
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self.input.as_bytes()[self.read_position];
        }
    }

    fn new_token(&self, token_type: TokenType, ch: u8) -> Token {
        Token::new(token_type, String::from_utf8(vec![ch]).unwrap())
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'0'..=b'9' => return self.consume_number(),

            b'=' => match self.peek_char() {
                b'=' => {
                    let position = self.position;
                    self.read_char();
                    let literal = &self.input[position..self.read_position];
                    Token::new(TokenType::Eq, String::from(literal))
                }
                _ => self.new_token(TokenType::Assign, self.ch),
            },
            b'+' => self.new_token(TokenType::Plus, self.ch),
            b'-' => self.new_token(TokenType::Minus, self.ch),
            b'!' => match self.peek_char() {
                b'=' => {
                    let position = self.position;
                    self.read_char();
                    let literal = &self.input[position..self.read_position];
                    Token::new(TokenType::NotEq, String::from(literal))
                }
                _ => self.new_token(TokenType::Bang, self.ch),
            },
            b'*' => self.new_token(TokenType::Asterisk, self.ch),
            b'/' => self.new_token(TokenType::Slash, self.ch),
            b'<' => self.new_token(TokenType::Lt, self.ch),
            b'>' => self.new_token(TokenType::Gt, self.ch),

            b',' => self.new_token(TokenType::Comma, self.ch),
            b';' => self.new_token(TokenType::Semicolon, self.ch),

            b'(' => self.new_token(TokenType::LParen, self.ch),
            b')' => self.new_token(TokenType::RParen, self.ch),
            b'{' => self.new_token(TokenType::LBrace, self.ch),
            b'}' => self.new_token(TokenType::RBrace, self.ch),

            0 => Token::new(TokenType::Eof, String::from("")),
            _ => {
                if self.is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = lookup_ident(literal);
                    return Token::new(token_type, String::from(literal));
                } else {
                    self.new_token(TokenType::Illegal, self.ch)
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        &self.input[position..self.position]
    }

    fn is_letter(&self, ch: u8) -> bool {
        matches!(ch, b'a'..=b'z' | b'A'..=b'Z' | b'_')
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.ch, b' ' | b'\t' | b'\n') {
            self.read_char();
        }
    }

    fn consume_number(&mut self) -> Token {
        let position = self.position;
        while matches!(self.ch, b'0'..=b'9') {
            self.read_char();
        }
        let literal = &self.input[position..self.position];
        Token::new(TokenType::Int, String::from(literal))
    }
}
