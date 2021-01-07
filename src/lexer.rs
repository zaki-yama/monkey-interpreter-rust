use crate::Token;

#[test]
fn test_next_token() {
    let input = String::from("=+(){},;");

    let mut l = Lexer::new(&input);
    let tests = vec![
        Token::Assign,
        Token::Plus,
        Token::LParen,
        Token::RParen,
        Token::LBrace,
        Token::RBrace,
        Token::Comma,
        Token::Semicolon,
    ];

    for expected in tests {
        let token = l.next_token();

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
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            b'0'..=b'9' => Token::Int,

            b'=' => Token::Assign,
            b'+' => Token::Plus,

            b',' => Token::Comma,
            b';' => Token::Semicolon,

            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,

            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();
        tok
    }
}
