use thiserror::Error;

use crate::{
    ast::{Expression, Precedence, Program, Statement},
    lexer::Lexer,
    token::{Token, TokenType},
};

#[cfg(test)]
mod tests {
    use std::vec;

    use super::Parser;
    use crate::{
        ast::{Expression::*, Program, Statement},
        lexer::Lexer,
    };

    fn check_parse_errors(p: &Parser) {
        if p.errors.len() == 0 {
            return;
        }

        println!("parser has {} errors", p.errors.len());
        for error in &p.errors {
            println!("parser error: {}", error);
        }
        panic!("test failed");
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar";

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);

        if program.statements.len() != 1 {
            panic!(
                "program has not enough statements. got {}",
                program.statements.len()
            );
        }

        let statement = &program.statements[0];

        let expression = match statement {
            Statement::Expression(expression) => expression,
            _ => {
                panic!("program.statements[0] is not expression. got {}", statement);
            }
        };

        let value = match expression {
            Identifier(value) => value,
            _ => panic!("expression is not Identifier. got {}", expression),
        };
        assert_eq!("foobar", value);
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);

        if program.statements.len() != 1 {
            panic!(
                "program has not enough statements. got {}",
                program.statements.len()
            );
        }

        let statement = &program.statements[0];

        let expression = match statement {
            Statement::Expression(expression) => expression,
            _ => {
                panic!("program.statements[0] is not expression. got {}", statement);
            }
        };

        let value = match expression {
            IntegerLiteral(value) => *value,
            _ => panic!("expression is not Identifier. got {}", expression),
        };
        assert_eq!(5, value);
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_string() {
        let program = Program {
            statements: vec![Statement::Let {
                name: Identifier(String::from("myVar")),
                // value: Identifier(String::from("anotherVar"))
            }],
        };
        assert_eq!("let myVar = anotherValue;", program.to_string());
    }

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
        check_parse_errors(&parser);

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

    #[test]
    #[should_panic]
    fn test_let_statements_errors() {
        let input = "
    let x 5;
    let = 10;
    let 838383;
    ";

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        parser.parse_program();
        check_parse_errors(&parser);
    }

    #[test]
    fn test_return_statements() {
        let input = "
    return 5;
    return 10;
    return 838383;
    ";

        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);

        if program.statements.len() != 3 {
            panic!(
                "program.statements does not contain 3 statements. got={}",
                program.statements.len()
            );
        }

        let expected = vec![Statement::Return, Statement::Return, Statement::Return];
        assert_eq!(expected, program.statements);
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("expected next token to be {expected:?}, got {actual:?} instead")]
    UnexpectedToken {
        expected: TokenType,
        actual: TokenType,
    },
    #[error("could not parse {0} as integer")]
    FailedToParseInteger(String),
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<ParseError>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Self {
            lexer,
            current_token: Token::new(TokenType::Eof, String::from("")),
            peek_token: Token::new(TokenType::Eof, String::from("")),
            errors: vec![],
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
            TokenType::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let identifier = Expression::Identifier(self.current_token.literal.clone());
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

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let statement = Statement::Return;
        self.next_token();

        // TODO: セミコロンに遭遇するまで式を読み飛ばしてしまっている
        while !self.current_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(statement)
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = self.parse_expression(Precedence::Lowest).unwrap();

        let statement = Statement::Expression(expression);

        if self.peek_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(statement)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // トークンタイプにひもづけられた構文解析関数を呼び出す
        let prefix = match self.current_token.token_type {
            TokenType::Ident => Expression::Identifier(self.parse_identifier()),
            TokenType::Int => Expression::IntegerLiteral(self.parse_integer_literal()),
            _ => return None,
        };
        Some(prefix)
    }

    fn parse_identifier(&self) -> String {
        self.current_token.literal.clone()
    }

    fn parse_integer_literal(&mut self) -> i64 {
        match self.current_token.literal.parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                self.errors.push(ParseError::FailedToParseInteger(
                    self.current_token.literal.clone(),
                ));
                // FIXME: 適当な値を返すのではなく戻り値自体を Result にした方がよさそう
                0
            }
        }
    }

    fn current_token_is(&self, t: TokenType) -> bool {
        self.current_token.token_type == t
    }

    fn peek_token_is(&self, t: &TokenType) -> bool {
        self.peek_token.token_type == *t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(&t) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t);
            return false;
        }
    }

    fn peek_error(&mut self, t: TokenType) {
        self.errors.push(ParseError::UnexpectedToken {
            expected: t,
            actual: self.peek_token.token_type.clone(),
        });
    }
}
