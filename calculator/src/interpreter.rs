use crate::common::{Expr, TokenLiteral, TokenType};

macro_rules! evaluate_op {
    ($op:tt $right:ident) => {
        match $right {
            TokenLiteral::Number(float64) => ($op float64).into(),
            _ => TokenLiteral::None
        }
    };
    ($left:ident $op:tt $right:ident) => {
        {
            match ($left, $right) {
                (TokenLiteral::Number(left), TokenLiteral::Number(right)) =>  (left $op right).into(),
                (_, _) => TokenLiteral::None
            }
        }
    };
}

pub fn evaluate(expression: Expr) -> TokenLiteral {
    match expression {
        Expr::Binary {
            left,
            operator,
            right,
        } => evaluate_binary(*left, operator, *right),
        Expr::Unary { operator, right } => evaluate_unary(operator, *right),
        Expr::Grouping { expression } => evaluate(*expression),
        Expr::Literal { value } => value,
    }
}

fn evaluate_binary(left: Expr, operator: TokenType, right: Expr) -> TokenLiteral {
    let left_literal = evaluate(left);
    let right_literal = evaluate(right);
    match operator {
        TokenType::Add => evaluate_op!(left_literal + right_literal),
        TokenType::Sub => evaluate_op!(left_literal - right_literal),
        TokenType::Div => evaluate_op!(left_literal / right_literal),
        TokenType::Mul => evaluate_op!(left_literal * right_literal),
        _ => TokenLiteral::None,
    }
}

fn evaluate_unary(operator: TokenType, right: Expr) -> TokenLiteral {
    let right_literal = evaluate(right);

    match operator {
        TokenType::Sub => evaluate_op!(-right_literal),
        _ => TokenLiteral::None,
    }
}
