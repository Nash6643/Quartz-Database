use crate::database::Database;
use crate::query::ast::Query;
use crate::query::planner::Plan;

pub fn execute(
    db: &mut Database,
    plan: Plan,
) {
    match plan {

        Plan::TableScan(query) => {
            println!("Execution Plan: Table Scan");
            execute_query(db, query);
        }

        Plan::IndexScan(query) => {

            println!("Execution Plan: Index Scan");
        
            match query {
        
                Query::Find {
                    table,
                    conditions,
                } => {
        
                    // For now, if there is exactly one condition,
                    // use the indexed lookup.
        
                    if conditions.len() == 1 {
        
                        let condition = &conditions[0];
        
                        db.select_using_index(
                            &table,
                            &condition.column,
                            &condition.value,
                        );
        
                    } else {
        
                        db.find(
                            &table,
                            &conditions,
                        );
        
                    }
        
                }
        
                other => {
        
                    execute_query(db, other);
        
                }
        
            }
        
        }

    }
}

fn execute_query(
    db: &mut Database,
    query: Query,
) {
    match query {

        Query::Make {
            table,
            columns,
        } => {
            db.create_table(table, columns);
        }

        Query::Add {
            table,
            values,
        } => {
            db.insert_row(&table, values);
        }

        Query::Show {
            table,
            columns,
        } => {
        
            db.print_table(
                &table,
                columns.as_deref(),
            );
        
        }

        Query::Find {
            table,
            conditions,
        } => {
            db.find(
                &table,
                &conditions,
            );
        }

        Query::Change {
            table,
            column,
            value,
            where_column,
            where_value,
        } => {
            db.update_where(
                &table,
                &column,
                &value,
                &where_column,
                &where_value,
            );
        }

        Query::Remove {
            table,
            column,
            value,
        } => {
            db.delete_where(
                &table,
                &column,
                &value,
            );
        }

        Query::Plan {
            table,
            column,
            value,
        } => {
            db.explain_select(
                &table,
                &column,
                &value,
            );
        }

        Query::Save {
            filename,
        } => {
            db.save(&filename);
        }

        Query::Load {
            filename,
        } => {
            db.load(&filename);
        }

        Query::Count {
            table,
        } => {
            db.count(&table);
        }

        Query::Sum {
            table,
            column,
        } => {
            db.sum(
                &table,
                &column,
            );
        }

        Query::Avg {
            table,
            column,
        } => {
            db.avg(
                &table,
                &column,
            );
        }

        Query::Min {
            table,
            column,
        } => {
            db.min(
                &table,
                &column,
            );
        }

        Query::Max {
            table,
            column,
        } => {
            db.max(
                &table,
                &column,
            );
        }

        Query::Group {
            table,
            group_column,
            aggregate,
            target_column,
        } => {
            db.group_by(
                &table,
                &group_column,
                &aggregate,
                target_column.as_deref(),
            );
        }

        Query::Begin => {
            db.begin_transaction();
        }
        
        Query::Commit => {
            db.commit_transaction();
        }
        
        Query::Rollback => {
            db.rollback_transaction();
        }

        Query::Join {
            left_table,
            right_table,
            left_column,
            right_column,
        } => {
        
            db.join(
                &left_table,
                &right_table,
                &left_column,
                &right_column,
            );
        
        }

        Query::Help => {
            println!("================ QuartzDB Commands ================");
            println!("MAKE table column:type[:constraint] ...");
            println!("ADD table value1 value2 ...");
            println!("SHOW table");
            println!("FIND table column value");
            println!("CHANGE table column value IF column value");
            println!("REMOVE table column value");
            println!("PLAN FIND table column value");
            println!("COUNT table");
            println!("SUM table column");
            println!("AVG table column");
            println!("MIN table column");
            println!("MAX table column");
            println!("GROUP table column AGGREGATE target_column");
            println!("SAVE filename");
            println!("LOAD filename");
            println!("HELP");
            println!("EXIT");
        }

        Query::Exit => {
            println!("Goodbye!");
        }
    }
}