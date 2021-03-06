
pub enum Node {
    Statement(Box<Statement>),
    Expression(Box<Expression>),
    Program(Box<Program>)
}

pub enum Statement {
    Let(Box<LetStatement>)
}

pub enum Expression {

}

pub struct Program {
    pub statments: Vec<Statement>
}

impl Program {
    pub fn new() -> Program {
        Program {
            statments: Vec::new(),
        }
    }
}

pub struct LetStatement {
    pub name: String,
   // pub value: Expression
}