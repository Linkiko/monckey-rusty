use std::fmt;

#[derive(PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers + Litterals
    Identifier(String),
    Int(i32),

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    //Delimiters
    Comma,
    Semicolon,

    LParenthesis,
    RParenthesis,
    LBracket,
    RBracket,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,

    Eq,
    NotEq,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Illegal => write!(f, "Token::Illegal"),
            Token::EOF => write!(f, "Token::EOF"),
            Token::Identifier(string) => write!(f, "Token::Identifier({})", string),
            Token::Int(x) => write!(f, "Token::Int({})", x),
            Token::Assign => write!(f, "Token::Assign (=)"),
            Token::Plus => write!(f, "Token::Plus (+)"),
            Token::Minus => write!(f, "Token::Minus (-) "),
            Token::Bang => write!(f, "Token::Bang (!)"),
            Token::Asterisk => write!(f, "Token::Asterisk (*)"),
            Token::Slash => write!(f, "Token::Slash (/)"),
            Token::Lt => write!(f, "Token::Lower Than (<)"),
            Token::Gt => write!(f, "Token::Greater Than (>)"),
            Token::Comma => write!(f, "Token::Comma (,)"),
            Token::Semicolon => write!(f, "Token::Semicolon (;)"),
            Token::LParenthesis => write!(f, "Token::LParenthesis ( "),
            Token::RParenthesis => write!(f, "Token::RParenthesis )"),
            Token::LBracket => write!(f, "Token::LBracket ["),
            Token::RBracket => write!(f, "Token::RBracket ]"),
            Token::Function => write!(f, "Token::Function"),
            Token::Let => write!(f, "Token::Let"),
            Token::True => write!(f, "Token::True"),
            Token::False => write!(f, "Token::False"),
            Token::If => write!(f, "Token::If"),
            Token::Else => write!(f, "Token::Else"),
            Token::Return => write!(f, "Token::Return"),
            Token::Eq => write!(f, "Token::Eq (==)"),
            Token::NotEq => write!(f, "Token::NotEq (!=)"),
        }
    }
}

pub fn lookup_identifier(identifier: String) -> Token {
    match identifier.as_str() {
        "let" => Token::Let,
        "fn" => Token::Function,
        "true" => Token::True,
        "false" => Token::False,
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Identifier(identifier),
    }
}
