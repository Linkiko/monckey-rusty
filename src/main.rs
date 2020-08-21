use std::env;
use std::process::exit;
use std::fs;

mod token;
use token::{Token};

mod lexer;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Error: Wrong number of inputs");
        exit(1);
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
