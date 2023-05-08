use crate::{common::TokenLiteral, interpreter::evaluate, lexer::Lexer, parser::Parser};

pub fn interpret(buf: &[u8], len: usize) -> f64 {
    let tokens = Lexer::run(buf, len);
    let expression = Parser::run(tokens);
    let result = evaluate(expression);
    match result {
        TokenLiteral::Number(float64) => float64,
        TokenLiteral::None => panic!("Interpreter failed to get a result"),
    }
}

#[cfg(test)]
mod tests {
    use crate::calculator::interpret;

    #[test]
    fn calculator_test_success() {
        let buf = b"12*(10-(100/10-15))";
        let result = interpret(buf, buf.len());
        let expected = 180.;
        assert_eq!(result, expected);
    }
}
