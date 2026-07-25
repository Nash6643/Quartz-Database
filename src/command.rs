use crate::database::Database;
use crate::query::lexer;
use crate::query::parser;
use crate::query::planner;
use crate::query::executor;

pub fn execute_command(
    db: &mut Database,
    input: &str,
) {
    // Step 1: Convert the user's input into tokens
    let tokens = lexer::lex(input);

    // Step 2: Parse the tokens into a Query AST
    let query = match parser::parse(&tokens) {
        Ok(query) => query,
        Err(error) => {
            println!("{}", error);
            return;
        }
    };

    // Step 3: Execute the query
    let plan = planner::build(db, query);

    executor::execute(db, plan);
}