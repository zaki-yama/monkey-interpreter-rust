use crate::{
    ast::{Expression::*, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

#[test]
fn test_let_statements() {
    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";

    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();
    if program.statements.len() != 3 {
        panic!(
            "program.statements does not contain 3 statements. got={}",
            program.statements.len()
        );
    }

    let expected = vec![
        Statement::Let {
            name: Identifier(String::from("x")),
        },
        Statement::Let {
            name: Identifier(String::from("y")),
        },
        Statement::Let {
            name: Identifier(String::from("foobar")),
        },
    ];
    assert_eq!(expected, program.statements);
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::new(TokenType::Eof, String::from("")),
            peek_token: Token::new(TokenType::Eof, String::from("")),
        };

        parser.next_token();
        parser.next_token();

        parser
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = vec![];

        while self.current_token.token_type != TokenType::Eof {
            match self.parse_statement() {
                Some(statement) => statements.push(statement),
                None => {}
            }
            self.next_token();
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.token_type {
            TokenType::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let identifier = Identifier(self.current_token.literal.clone());
        let statement = Statement::Let {
            name: identifier,
            // value,
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        // TODO: セミコロンに遭遇するまで式を読み飛ばしてしまっている
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(statement)
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type == t
    }

    fn peek_token_is(&self, t: TokenType) -> bool {
        self.peek_token.token_type == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}
