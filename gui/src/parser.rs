use crate::error::ParserError;

pub fn parse(operation: &str) -> Result<(), ParserError> {
    let mut num_paren = 0;
    if operation.len() == 0 {
        return Err(ParserError(
            "Expected an operation, got empty string".into(),
        ));
    }
    for character in operation.chars() {
        if !character.is_ascii_digit() {
            match character {
                '-' => {}
                '+' => {}
                '*' => {}
                '/' => {}
                '(' => num_paren += 1,
                ')' => num_paren -= 1,
                _ => return Err(ParserError(format!("Bad character {}", character))),
            }
        }
    }
    if num_paren != 0 {
        return Err(ParserError(
            "Parenthesis opened but not closed (or vice versa)".into(),
        ));
    }
    Ok(())
}
