use crate::database::Database;
use crate::query::ast::Query;

#[derive(Debug)]
pub enum Plan {
    TableScan(Query),
    IndexScan(Query),
}

pub fn build(
    db: &Database,
    query: Query,
) -> Plan {

    match &query {

        Query::Find {
            table,
            conditions,
        } => {
        
            if conditions.len() == 1 {
        
                let condition = &conditions[0];
        
                if db.has_index(
                    table,
                    &condition.column,
                ) {
        
                    println!("Planner: using INDEX SCAN");
        
                    return Plan::IndexScan(query);
        
                }
        
            }
        
            println!("Planner: using TABLE SCAN");
        
            Plan::TableScan(query)
        
        }
        _ => Plan::TableScan(query),
    }
}