mod command;
mod database;
mod repl;
mod query;
mod pager;
mod page;
mod catalog;
mod row_storage;

use database::Database;

fn main() {
    let mut db = Database::new();

    repl::start(&mut db);
}