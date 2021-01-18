use core::panic;

use crate::{
    ast::{Expression::*, Program, Statement},
    lexer::Lexer,
    token::Token,
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
    // for (i, identifier) in expected_identifier.iter().enumerate() {
    //     let statement = program.statements[i];
    //     assert_eq!(statement)
    //     if !test_let_statement(statement, identifier) {
    //         return;
    //     }
    // }
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
            current_token: Token::Eof,
            peek_token: Token::Eof,
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

        while self.current_token != Token::Eof {
            match self.parse_statement() {
                Some(statement) => statements.push(statement),
                None => {}
            }
            self.next_token();
        }

        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        match self.peek_token {
            Token::Ident(_) => self.next_token(),
            _ => return None,
        }

        let name = self.parse_identifier()?;

        let statement = Statement::Let {
            name: Identifier(name), // value:
        };

        // TODO: セミコロンに遭遇するまで式を読み飛ばしてしまっている
        while self.current_token != Token::Semicolon {
            self.next_token();
        }

        Some(statement)
    }

    fn parse_identifier(&self) -> Option<String> {
        match &self.current_token {
            Token::Ident(name) => return Some(name.clone()),
            _ => return None,
        }
    }

    fn expect_peek(&mut self, t: Token) -> bool {
        if self.peek_token == t {
            self.next_token();
            return true;
        } else {
            return false;
        }
    }
}
