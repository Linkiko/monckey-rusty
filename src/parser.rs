use crate::ast;
use crate::lexer::Lexer;
use crate::Token;

struct Parser<'a> {
    lexer: Lexer<'a>,
    curr_token: Token,
    peek_token: Token
}

impl Parser<'_> {
    pub fn new(mut lexer: Lexer) -> Parser {
        let curr_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer,
            curr_token,
            peek_token
        }
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();
        while self.curr_token != Token::EOF {
            match self.parse_statement() {
                Ok(statment) => program.statments.push(statment),
                Err(error) => println!("{}", error)
            }
            self.next_token();    
        }
        program
    }

    fn parse_statement(&mut self) -> Result<ast::Statement, String> {
        match self.curr_token {
            Token::Let => self.parse_let_statement(),
            _ => Err(String::from("Invalid Statement"))
        }
    }

    fn parse_let_statement(&mut self) -> Result<ast::Statement, String> {
        if !self.expect_peek(Token::Identifier) {
            Err(String::from("Parsing error"));
        } 
    }

    fn curr_token_is(&self, token: Token) -> bool {
        self.curr_token == token
    }

    fn peek_token_is(&self, token: Token) -> bool {
        self.peek_token == token
    }

    fn expect_peek(&self, token: Token) -> bool {
        if self.peek_token_is(token) {
            self.next_token();
            true
        }
        else {
            false
        }
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
}