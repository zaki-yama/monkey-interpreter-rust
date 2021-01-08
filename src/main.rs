use std::io::{self, Write};

use monkey_interpreter_rust::{lexer::Lexer, token::Token};

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let mut lexer = Lexer::new(&input);
        while let tok = lexer.next_token() {
            if tok == Token::Eof {
                break;
            }
            println!("{:?} ", tok);
        }
    }
}
