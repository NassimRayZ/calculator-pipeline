use crate::common::{Token, TokenLiteral, TokenType};

pub struct Lexer<'a> {
    source: &'a [u8],
    len: usize,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a [u8], len: usize) -> Self {
        Self {
            source,
            len,
            current: 0,
            start: 0,
            tokens: vec![],
        }
    }
    pub fn run(source: &'a [u8], len: usize) -> Vec<Token> {
        let lexer = Self::new(source, len);
        lexer.scan_tokens()
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return 0;
        }
        self.source[self.current]
    }
    fn next(&mut self) -> u8 {
        self.current += 1;
        self.source[self.current - 1]
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.len
    }
    fn scan_tokens(mut self) -> Vec<Token> {
        loop {
            if self.is_at_end() {
                break self.tokens;
            }
            self.scan_token();
            self.start = self.current;
        }
    }
    fn scan_token(&mut self) {
        let byte = self.next();
        match byte {
            b'+' => self.add_token(TokenType::Add),
            b'-' => self.add_token(TokenType::Sub),
            b'*' => self.add_token(TokenType::Mul),
            b'/' => self.add_token(TokenType::Div),
            b'(' => self.add_token(TokenType::Lparen),
            b')' => self.add_token(TokenType::Rparen),
            _ => self.add_token_number(),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type, TokenLiteral::None));
    }
    fn add_token_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.next();
        }
        if self.peek() == b'.' {
            self.next();
            while self.peek().is_ascii_digit() {
                self.next();
            }
        }
        let number = std::str::from_utf8(&self.source[self.start..self.current]).unwrap();
        let number = number.parse::<f64>().unwrap();
        self.tokens
            .push(Token::new(TokenType::Number, TokenLiteral::Number(number)));
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{Token, TokenLiteral, TokenType};

    use super::Lexer;

    #[test]
    fn lexer_test_succes() {
        let buf = b"12.+10.12";
        let lexer = Lexer::new(buf, buf.len());
        let result = lexer.scan_tokens();

        let expected = vec![
            Token::new(TokenType::Number, TokenLiteral::Number(12.)),
            Token::new(TokenType::Add, TokenLiteral::None),
            Token::new(TokenType::Number, TokenLiteral::Number(10.12)),
        ];
        assert_eq!(result, expected);
    }
}
