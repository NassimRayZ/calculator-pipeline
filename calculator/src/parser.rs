use crate::common::TokenLiteral;
#[allow(unused_imports)]
use crate::common::{Expr, Token, TokenType};
use anyhow::{anyhow, Result};

macro_rules! match_type {
    ($parser:ident, $($type:tt),*) => {{
        let token_type = &$parser.peek().token_type;
        let mut result = false;
        if $(token_type == &TokenType::$type ||)* false {
            $parser.next();
            result = true
        }
        result
    }};
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn run(tokens: Vec<Token>) -> Expr {
        let mut parser = Self::new(tokens);
        match parser.parse() {
            Ok(expr) => expr,
            Err(e) => panic!("Parser Failed: {:#?}", e),
        }
    }

    pub fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or(&Token {
            token_type: TokenType::Eof,
            literal: TokenLiteral::None,
        })
    }
    pub fn next(&mut self) -> &Token {
        self.current += 1;
        self.tokens.get(self.current - 1).unwrap()
    }
    fn previous(&mut self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }

    // expression := term
    fn expression(&mut self) -> Result<Expr> {
        self.term()
    }
    // term := factor (('+' | '-') factor)*
    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while match_type!(self, Add, Sub) {
            let operator = self.previous().token_type;
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: self.factor()?.into(),
            };
        }
        Ok(expr)
    }

    // factor := unary (('*' | '/') unary)*
    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.unary()?;
        while match_type!(self, Mul, Div) {
            let operator = self.previous().token_type;
            expr = Expr::Binary {
                left: expr.into(),
                operator,
                right: self.unary()?.into(),
            }
        }
        Ok(expr)
    }

    // unary := ('-')? unary
    //        | primary
    fn unary(&mut self) -> Result<Expr> {
        if match_type!(self, Sub) {
            let operator = self.previous().token_type;
            return Ok(Expr::Unary {
                operator,
                right: self.unary()?.into(),
            });
        }
        self.primary()
    }

    // primary := number
    //          | '(' expression ')'
    fn primary(&mut self) -> Result<Expr> {
        if match_type!(self, Number) {
            return Ok(Expr::Literal {
                value: self.previous().literal,
            });
        }
        if match_type!(self, Lparen) {
            let expr = self.expression()?;
            if match_type!(self, Rparen) {
                return Ok(Expr::Grouping {
                    expression: expr.into(),
                });
            }
            return Err(anyhow!("Expected ')' after expression"));
        }
        return Err(anyhow!("Unknow TokenType {:?}", self.previous().token_type));
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{Expr, Token, TokenLiteral, TokenType};

    use super::Parser;

    #[test]
    fn macro_types_success() {
        let mut parser = Parser::new(vec![
            Token::new(TokenType::Eof, TokenLiteral::None),
            Token::new(TokenType::Add, TokenLiteral::None),
            Token::new(TokenType::Sub, TokenLiteral::None),
        ]);
        let result: bool = match_type!(parser, Add);
        assert_eq!(result, false);
    }

    #[test]
    fn parser_success() {
        let mut parser = Parser::new(vec![
            Token::new(TokenType::Number, TokenLiteral::Number(12.)),
            Token::new(TokenType::Add, TokenLiteral::None),
            Token::new(TokenType::Number, TokenLiteral::Number(12.)),
        ]);
        let result = parser.parse().unwrap();
        let expected = Expr::Binary {
            left: Expr::Literal {
                value: TokenLiteral::Number(12.),
            }
            .into(),
            operator: TokenType::Add,
            right: Expr::Literal {
                value: TokenLiteral::Number(12.),
            }
            .into(),
        };

        assert_eq!(expected, result);
    }
}
