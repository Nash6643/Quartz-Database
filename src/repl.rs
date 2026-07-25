use crate::database::Database;
use crate::query::executor;
use crate::query::lexer;
use crate::query::parser;
use crate::query::planner;

use std::io::{self, Write};

pub fn start(db: &mut Database) {
    println!("==============================");
    println!("       QuartzDB v0.5");
    println!("==============================");
    println!("Type HELP for commands.");

    loop {
        print!("QuartzDB > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let tokens = lexer::lex(input);

        let query = match parser::parse(&tokens) {
            Ok(query) => query,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        if matches!(query, crate::query::ast::Query::Exit) {
            println!("Goodbye!");
            break;
        }

        let plan = planner::build(db, query);

        executor::execute(db, plan);
    }
}