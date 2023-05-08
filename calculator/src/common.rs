#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum TokenType {
    Add,
    Sub,
    Div,
    Mul,
    Lparen,
    Rparen,
    Number,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenLiteral {
    Number(f64),
    None,
}

impl From<f64> for TokenLiteral {
    fn from(value: f64) -> Self {
        TokenLiteral::Number(value)
    }
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: TokenLiteral,
}

impl Token {
    pub fn new(token_type: TokenType, literal: TokenLiteral) -> Self {
        Self {
            token_type,
            literal,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: TokenType,
        right: Box<Expr>,
    },
    Unary {
        operator: TokenType,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: TokenLiteral,
    },
}
