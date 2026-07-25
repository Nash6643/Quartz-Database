use crate::query::ast::{
    Query,
    Condition,
    Operator,
    Logic,
};
use crate::query::token::Token;
use crate::query::helpers::*;


pub fn parse(tokens: &[Token]) -> Result<Query, String> {

    if tokens.is_empty() {
        return Err("Empty command.".to_string());
    }

    match &tokens[0] {

        Token::Make => parse_make(tokens),

        Token::Group => parse_group(tokens),

        Token::Add => parse_add(tokens),

        Token::Show => parse_show(tokens),

        Token::Find => parse_find(tokens),

        Token::Change => parse_change(tokens),

        Token::Remove => parse_remove(tokens),

        Token::Plan => parse_plan(tokens),

        Token::Save => parse_save(tokens),

        Token::Load => parse_load(tokens),

        Token::Help => Ok(Query::Help),

        Token::Exit => Ok(Query::Exit),

        Token::Count => parse_count(tokens),

        Token::Sum => parse_sum(tokens),

        Token::Avg => parse_avg(tokens),

        Token::Min => parse_min(tokens),

        Token::Join => parse_join(tokens),

        Token::Max => parse_max(tokens),

        Token::Begin => Ok(Query::Begin),

        Token::Commit => Ok(Query::Commit),

        Token::Rollback => Ok(Query::Rollback),

        _ => Err("Unknown command.".to_string()),
    }
}


fn parse_show(tokens: &[Token]) -> Result<Query, String> {

    use crate::query::helpers::*;

    if tokens.len() < 2 {
        return Err("Usage: SHOW table [columns...]".to_string());
    }

    let table = identifier(&tokens[1])?;

    let columns =
        if tokens.len() > 2 {

            let mut list = Vec::new();

            for token in &tokens[2..] {
                list.push(identifier(token)?);
            }

            Some(list)

        } else {

            None

        };

    Ok(
        Query::Show {
            table,
            columns,
        }
    )
}

fn parse_find(tokens: &[Token]) -> Result<Query, String> {

    use crate::query::helpers::*;

    if tokens.len() < 4 {
        return Err(
            "Usage: FIND table column operator value [AND/OR ...]".to_string()
        );
    }

    let table = identifier(&tokens[1])?;

    let mut conditions = Vec::new();

    let mut i = 2;

    while i < tokens.len() {

        // Column
        let column = identifier(&tokens[i])?;
        i += 1;

        if i >= tokens.len() {
            return Err("Expected operator.".to_string());
        }

 // BETWEEN is handled separately
if matches!(&tokens[i], Token::Between) {

    i += 1;

    if i + 1 >= tokens.len() {
        return Err("BETWEEN requires two values.".to_string());
    }

    let first = token_to_value(&tokens[i])?;
    i += 1;

    let second = token_to_value(&tokens[i])?;
    i += 1;

    conditions.push(Condition {
        column: column.clone(),
        operator: Operator::GreaterEqual,
        value: first,
        logic: Some(Logic::And),
    });

    conditions.push(Condition {
        column,
        operator: Operator::LessEqual,
        value: second,
        logic: None,
    });

    continue;
}

// IN operator
if matches!(&tokens[i], Token::In) {

    i += 1;

    while i < tokens.len() {

        let value = token_to_value(&tokens[i])?;
        i += 1;

        let has_more = i < tokens.len();

        conditions.push(Condition {
            column: column.clone(),
            operator: Operator::Equal,
            value,
            logic: if has_more {
                Some(Logic::Or)
            } else {
                None
            },
        });

    }

    break;
}

// Normal operators
let operator = match &tokens[i] {

    Token::Equal => Operator::Equal,

    Token::NotEqual => Operator::NotEqual,

    Token::Greater => Operator::Greater,

    Token::Less => Operator::Less,

    Token::GreaterEqual => Operator::GreaterEqual,

    Token::LessEqual => Operator::LessEqual,

    Token::Like => Operator::Like,

    _ => return Err("Expected comparison operator.".to_string()),

};

        i += 1;

        if i >= tokens.len() {
            return Err("Expected value.".to_string());
        }

        // Value
        let value = token_to_value(&tokens[i])?;
        i += 1;

        // Optional AND / OR
        let logic =
            if i < tokens.len() {

                match &tokens[i] {

                    Token::And => {
                        i += 1;
                        Some(Logic::And)
                    }

                    Token::Or => {
                        i += 1;
                        Some(Logic::Or)
                    }

                    _ => None,

                }

            } else {
                None
            };

        conditions.push(

            Condition {

                column,

                operator,

                value,

                logic,

            }

        );

    }

    Ok(

        Query::Find {

            table,

            conditions,

        }

    )

}

fn parse_add(tokens: &[Token]) -> Result<Query, String> {
    if tokens.len() < 3 {
        return Err("Usage: ADD table values...".to_string());
    }

    let table = identifier(&tokens[1])?;

    let mut values = Vec::new();

    for token in &tokens[2..] {
        values.push(token_to_value(token)?);
    }

    Ok(Query::Add { table, values })
}

fn parse_make(tokens: &[Token]) -> Result<Query, String> {

    if tokens.len() < 3 {
        return Err(
            "Usage: MAKE table column:type...".to_string()
        );
    }

    // Table name
    let table = match &tokens[1] {
        Token::Identifier(name) => name.clone(),
        _ => return Err("Expected table name.".to_string()),
    };

    let mut columns = Vec::new();

    // Every remaining token is a column definition
    for token in &tokens[2..] {

        match token {

            Token::Identifier(column) => {
                columns.push(column.clone());
            }

            _ => {
                return Err(
                    "Expected column definition.".to_string()
                );
            }

        }

    }

    Ok(
        Query::Make {
            table,
            columns,
        }
    )
}


fn parse_change(tokens: &[Token]) -> Result<Query, String> {
    if tokens.len() != 7 {
        return Err("Usage: CHANGE table column value IF column value".to_string());
    }

    if !matches!(tokens[4], Token::If) {
        return Err("Expected IF.".to_string());
    }

    Ok(Query::Change {
        table: identifier(&tokens[1])?,
        column: identifier(&tokens[2])?,
        value: token_to_value(&tokens[3])?,
        where_column: identifier(&tokens[5])?,
        where_value: token_to_value(&tokens[6])?,
    })
}


fn parse_remove(tokens: &[Token]) -> Result<Query, String> {
    if tokens.len() != 4 {
        return Err("Usage: REMOVE table column value".to_string());
    }

    Ok(Query::Remove {
        table: identifier(&tokens[1])?,
        column: identifier(&tokens[2])?,
        value: token_to_value(&tokens[3])?,
    })
}

fn parse_plan(tokens: &[Token]) -> Result<Query, String> {
    if tokens.len() != 5 {
        return Err("Usage: PLAN FIND table column value".to_string());
    }

    if !matches!(tokens[1], Token::Find) {
        return Err("Expected FIND.".to_string());
    }

    Ok(Query::Plan {
        table: identifier(&tokens[2])?,
        column: identifier(&tokens[3])?,
        value: token_to_value(&tokens[4])?,
    })
}


fn parse_save(tokens: &[Token]) -> Result<Query, String> {

    if tokens.len() != 2 {
        return Err(
            "Usage: SAVE filename".to_string()
        );
    }

    match &tokens[1] {

        Token::Identifier(file) => {

            Ok(Query::Save {
                filename: file.clone(),
            })

        }

        _ => Err("Expected filename.".to_string())
    }
}


fn parse_count(tokens: &[Token]) -> Result<Query, String> {

    if tokens.len() != 2 {
        return Err("Usage: COUNT table".to_string());
    }

    let table = identifier(&tokens[1])?;

    Ok(Query::Count {
        table,
    })
}

fn parse_sum(tokens: &[Token]) -> Result<Query, String> {

    if tokens.len() != 3 {
        return Err("Usage: SUM table column".to_string());
    }

    let table = identifier(&tokens[1])?;

    let column = identifier(&tokens[2])?;

    Ok(Query::Sum {
        table,
        column,
    })
}

fn parse_avg(tokens: &[Token]) -> Result<Query, String> {

    if tokens.len() != 3 {
        return Err("Usage: AVG table column".to_string());
    }

    let table = identifier(&tokens[1])?;

    let column = identifier(&tokens[2])?;

    Ok(Query::Avg {
        table,
        column,
    })
}

fn parse_min(tokens: &[Token]) -> Result<Query, String> {

    if tokens.len() != 3 {
        return Err("Usage: MIN table column".to_string());
    }

    let table = identifier(&tokens[1])?;

    let column = identifier(&tokens[2])?;

    Ok(Query::Min {
        table,
        column,
    })
}

fn parse_max(tokens: &[Token]) -> Result<Query, String> {

    if tokens.len() != 3 {
        return Err("Usage: MAX table column".to_string());
    }

    let table = identifier(&tokens[1])?;

    let column = identifier(&tokens[2])?;

    Ok(Query::Max {
        table,
        column,
    })
}

fn parse_group(tokens: &[Token]) -> Result<Query, String> {

    if tokens.len() != 4 && tokens.len() != 5 {
        return Err(
            "Usage: GROUP table group_column COUNT|SUM|AVG|MIN|MAX [column]"
                .to_string(),
        );
    }

    let table = identifier(&tokens[1])?;
    let group_column = identifier(&tokens[2])?;

    let aggregate = match &tokens[3] {
        Token::Count => "COUNT".to_string(),
        Token::Sum => "SUM".to_string(),
        Token::Avg => "AVG".to_string(),
        Token::Min => "MIN".to_string(),
        Token::Max => "MAX".to_string(),
        _ => return Err("Expected aggregate.".to_string()),
    };

    let target_column =
        if tokens.len() == 5 {
            Some(identifier(&tokens[4])?)
        } else {
            None
        };

    Ok(Query::Group {
        table,
        group_column,
        aggregate,
        target_column,
    })
}

//Parse join
fn parse_join(tokens: &[Token]) -> Result<Query, String> {

    use crate::query::helpers::*;

    if tokens.len() != 7 {
        return Err(
            "Usage: JOIN table1 table2 ON table1.column = table2.column".to_string()
        );
    }

    let left_table = identifier(&tokens[1])?;
    let right_table = identifier(&tokens[2])?;

    match &tokens[3] {
        Token::On => {}
        _ => return Err("Expected ON.".to_string()),
    }

    let left = identifier(&tokens[4])?;

    match &tokens[5] {
        Token::Equal => {}
        _ => return Err("Expected '='.".to_string()),
    }

    let right = identifier(&tokens[6])?;

    let left_parts: Vec<&str> = left.split('.').collect();
    let right_parts: Vec<&str> = right.split('.').collect();

    if left_parts.len() != 2 || right_parts.len() != 2 {
        return Err("JOIN columns must be table.column".to_string());
    }

    Ok(Query::Join {

        left_table,

        right_table,

        left_column: left_parts[1].to_string(),

        right_column: right_parts[1].to_string(),

    })

}
fn parse_load(tokens: &[Token]) -> Result<Query, String> {

    if tokens.len() != 2 {
        return Err(
            "Usage: LOAD filename".to_string()
        );
    }

    match &tokens[1] {

        Token::Identifier(file) => {

            Ok(Query::Load {
                filename: file.clone(),
            })

        }

        _ => Err("Expected filename.".to_string())
    }
}


