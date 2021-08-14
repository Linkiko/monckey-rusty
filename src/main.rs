use std::env;
use std::fs;

pub mod token;
use token::Token;

pub mod ast;
pub mod lexer;
mod parser;
pub mod repl;

fn titi() -> () {
    println!("titi");
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        repl::start();
    }
    let input = fs::read_to_string(args[1].to_string()).expect("Error: Filepath is not correct");
    let mut lex = lexer::Lexer::new(&input);

    loop {
        match lex.next_token() {
            Token::EOF => {
                println!("Fini");
                break;
            }
            tok => println!("{}", tok),
        }
    }

    Ok(())
}
