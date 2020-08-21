use std::fmt;

pub enum Token {
    Illegal,
    EOF,

    // Identifiers + Litterals
    Identifier(String),
    Int(i32),

    // Operators
    Assign,
    Plus,

    //Delimiters
    Comma,
    Semicolon,

    LParenthesis,
    RParenthesis,
    LBracket,
    RBracket,

    // Keywords
    Function,
    Let
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Illegal => write!(f, "Token::Illegal"),
            Token::EOF => write!(f, "Token::EOF"),
            Token::Identifier(string) => write!(f, "Token::Identifier({})", string),
            Token::Int(x) => write!(f, "Token::Int({})", x),
            Token::Assign => write!(f, "Token::Assign"),
            Token::Plus => write!(f, "Token::Plus"),
            Token::Comma => write!(f, "Token::Comma"),
            Token::Semicolon => write!(f, "Token::Semicolon"),
            Token::LParenthesis => write!(f, "Token::LParenthesis"),
            Token::RParenthesis => write!(f, "Token::RParenthesis"),
            Token::LBracket => write!(f, "Token::LBracket"),
            Token::RBracket => write!(f, "Token::RBracket"),
            Token::Function => write!(f, "Token::Function "),
            Token::Let => write!(f, "Token::Let"),
        }
    }
}

pub fn lookup_identifier(identifier: String) -> Token {
    match identifier.as_str() {
        "let" => Token::Let,
        "fn" => Token::Function,
        _ => Token::Identifier(identifier),
    }
}

