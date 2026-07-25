use crate::query::token::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for part in input.split_whitespace() {
        let upper = part.to_uppercase();

        match upper.as_str() {
            "MAKE" => tokens.push(Token::Make),
            "ADD" => tokens.push(Token::Add),
            "SHOW" => tokens.push(Token::Show),
            "FIND" => tokens.push(Token::Find),
            "CHANGE" => tokens.push(Token::Change),
            "REMOVE" => tokens.push(Token::Remove),
            "PLAN" => tokens.push(Token::Plan),
            "SAVE" => tokens.push(Token::Save),
            "LOAD" => tokens.push(Token::Load),
            "HELP" => tokens.push(Token::Help),
            "EXIT" => tokens.push(Token::Exit),
            "IF" => tokens.push(Token::If),
            
            // New operator mappings
            "=" => tokens.push(Token::Equal),
            "!=" => tokens.push(Token::NotEqual),
            ">" => tokens.push(Token::Greater),
            "<" => tokens.push(Token::Less),
            ">=" => tokens.push(Token::GreaterEqual),
            "<=" => tokens.push(Token::LessEqual),

            "AND" => tokens.push(Token::And),

            "OR" => tokens.push(Token::Or),

            "COUNT" => tokens.push(Token::Count),
            "SUM"   => tokens.push(Token::Sum),
            "AVG"   => tokens.push(Token::Avg),
            "MIN"   => tokens.push(Token::Min),
            "MAX"   => tokens.push(Token::Max),

            "GROUP" => tokens.push(Token::Group),

            "BETWEEN" => tokens.push(Token::Between),

            "JOIN" => tokens.push(Token::Join),
            "ON" => tokens.push(Token::On),

            "BEGIN" => tokens.push(Token::Begin),
            "COMMIT" => tokens.push(Token::Commit),
            "ROLLBACK" => tokens.push(Token::Rollback),  

            "IN" => tokens.push(Token::In),
            "LIKE" => tokens.push(Token::Like),

            _ => {
                if let Ok(number) = part.parse::<i64>() {
                    tokens.push(Token::Integer(number));
                } else {
                    tokens.push(Token::Identifier(part.to_string()));
                }
            }
        }
    }

    tokens
}