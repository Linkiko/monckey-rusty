use crate::lexer;
use crate::token::Token;
use std::io::{self, Write};

pub fn start() {
    let mut input = String::new();
    println!("Welcome to the Monckey interpretor");
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut lexer = lexer::Lexer::new(&input);
                let mut tokens: Vec<Token> = Vec::new();
                loop {
                    let token = lexer.next_token();
                    if token == Token::EOF {
                        tokens.push(token);
                        break;
                    }
                    tokens.push(token);
                }
                tokens.iter_mut().for_each(|token| println!("{}", token));
            }
            Err(error) => println!("{}", error),
        }
        println!("");
    }
}
