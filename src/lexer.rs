use crate::Token;
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
    ",
    );
    let mut l = Lexer::new(&input);
    let tests = vec![
        Token::Let,
        Token::Ident(String::from("five")),
        Token::Assign,
        Token::Int(5),
        Token::Semicolon,
        //
        Token::Let,
        Token::Ident(String::from("ten")),
        Token::Assign,
        Token::Int(10),
        Token::Semicolon,
        //
        Token::Let,
        Token::Ident(String::from("foo")),
        Token::Assign,
        Token::Function,
        Token::LParen,
        Token::Ident(String::from("foo")),
        Token::RParen,
        Token::LBrace,
        Token::Ident(String::from("foo")),
        Token::Plus,
        Token::Ident(String::from("foo")),
        Token::Semicolon,
        Token::RBrace,
        Token::Semicolon,
        // let result = add(five, ten);
        Token::Let,
        Token::Ident(String::from("foo")),
        Token::Assign,
        Token::Ident(String::from("foo")),
        Token::LParen,
        Token::Ident(String::from("foo")),
        Token::Comma,
        Token::Ident(String::from("foo")),
        Token::Comma,
        Token::RParen,
        Token::Semicolon,
        Token::Eof,
    ];

    for expected in tests {
        let token = l.next_token();
        debug!("token: {:?}", token);
        assert_eq!(expected, token);
    }
}

struct Lexer<'a> {
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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            b'0'..=b'9' => return self.consume_number(),

            b'=' => Token::Assign,
            b'+' => Token::Plus,

            b',' => Token::Comma,
            b';' => Token::Semicolon,

            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,

            0 => Token::Eof,
            _ => {
                if self.is_letter(self.ch) {
                    let literal = self.read_identifier();

                    return match literal {
                        "let" => Token::Let,
                        "fn" => Token::Function,
                        _ => Token::Ident(String::from(literal)),
                    };
                }
                Token::Illegal
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
        Token::Int(literal.parse::<i64>().unwrap())
    }
}
