use crate::database::Value;
use crate::query::token::Token;

/// Returns an identifier (table name, column name, filename, etc.)
pub fn identifier(token: &Token) -> Result<String, String> {
    match token {
        Token::Identifier(v) => Ok(v.clone()),
        _ => Err("Expected identifier.".to_string()),
    }
}

/// Converts a token into a typed Value.
pub fn token_to_value(token: &Token) -> Result<Value, String> {
    match token {
        Token::Integer(v) => Ok(Value::Integer(*v)),

        Token::Text(v) => Ok(Value::Text(v.clone())),

        Token::Identifier(v) => {
            if v.eq_ignore_ascii_case("true") {
                Ok(Value::Boolean(true))
            } else if v.eq_ignore_ascii_case("false") {
                Ok(Value::Boolean(false))
            } else {
                Ok(Value::Text(v.clone()))
            }
        }

        _ => Err("Expected value.".to_string()),
    }
}