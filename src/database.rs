use std::collections::HashMap;
use std::fmt;
use crate::pager::Pager;
use crate::catalog::Catalog;
use serde::{Serialize, Deserialize};
use crate::row_storage::deserialize_row;
use crate::pager::PAGE_SIZE;
use crate::query::ast::{
    Condition,
    Operator,
    Logic,
};





#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Integer(i64),
    Text(String),
    Boolean(bool),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{i}"),
            Value::Text(s) => write!(f, "{s}"),
            Value::Boolean(b) => write!(f, "{b}"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Integer,
    Text,
    Boolean,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Column {

    pub name: String,

    pub data_type: DataType,

    pub primary_key: bool,

    pub unique: bool,

    pub nullable: bool,

    pub foreign_table: Option<String>,

    pub foreign_column: Option<String>,

    pub cascade_delete: bool,

}
#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Row {
    pub values: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Table {
    pub columns: Vec<Column>,
    pub rows: Vec<Row>,

    // column_name -> value -> row indexes
    pub indexes: HashMap<String, HashMap<String, Vec<usize>>>,
}

#[derive(Serialize, Deserialize,Clone)]
pub struct Database {
    pub tables: HashMap<String, Table>,

    pub catalog: Catalog,

    pub transaction_backup: Option<HashMap<String, Table>>,

    #[serde(skip)]
    pub pager: Pager,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
            transaction_backup: None,
            catalog: Catalog::new(),
            pager: Pager::new("quartz.db"),
        }
    }
//Create table
// Create table
pub fn create_table(
    &mut self,
    name: String,
    column_defs: Vec<String>,
) {
    let mut columns = Vec::new();

    for definition in column_defs {

        let parts: Vec<&str> = definition.split(':').collect();

        if parts.len() < 2 {
            println!("Invalid column definition '{}'", definition);
            return;
        }

        let data_type = match parts[1].to_lowercase().as_str() {

            "int" | "integer" => DataType::Integer,

            "text" | "string" => DataType::Text,

            "bool" | "boolean" => DataType::Boolean,

            _ => {
                println!("Unknown type '{}'", parts[1]);
                return;
            }

        };

        let mut primary_key = false;
        let mut unique = false;
        let mut nullable = true;

        let mut foreign_table = None;
        let mut foreign_column = None;

        let mut cascade_delete = false;

        for constraint in &parts[2..] {

            match constraint.to_lowercase().as_str() {

                "pk" | "primary" => {
                    primary_key = true;
                    unique = true;
                    nullable = false;
                }

                "unique" => {
                    unique = true;
                }

                "notnull" | "not_null" => {
                    nullable = false;
                }

                _ if constraint.starts_with("fk(") && constraint.ends_with(')') => {

                    let reference = &constraint[3..constraint.len() - 1];

                    let pieces: Vec<&str> = reference.split('.').collect();

                    if pieces.len() != 2 {
                        println!("Invalid foreign key '{}'", constraint);
                        return;
                    }

                    foreign_table = Some(pieces[0].to_string());
                    foreign_column = Some(pieces[1].to_string());

                }

                "cascade" => {
                cascade_delete = true;
            }

                _ => {
                    println!("Unknown constraint '{}'", constraint);
                    return;
                }

            }

        }

columns.push(Column {
    name: parts[0].to_string(),
    data_type,

    primary_key,
    unique,
    nullable,

    foreign_table,
    foreign_column,
    cascade_delete,
});

    }
    let root_page = self.pager.allocate_page();

        self.catalog.register_table(
            name.clone(),
            root_page,
        );

        println!(
            "Allocated root page {} for '{}'",
            root_page,
            name
        );

    self.tables.insert(

        name.clone(),

        Table {

            columns,

            rows: Vec::new(),

            indexes: HashMap::new(),

        },

    );
    let page = self.pager.allocate_page();

    self.catalog.register_table(
        name.clone(),
        page,
    );

    println!("Table '{}' created.", name);

}
    fn parse_value(value: &str) -> Value {
        if let Ok(number) = value.parse::<i64>() {
            Value::Integer(number)
        } else if value.eq_ignore_ascii_case("true") {
            Value::Boolean(true)
        } else if value.eq_ignore_ascii_case("false") {
            Value::Boolean(false)
        } else {
            Value::Text(value.to_string())
        }
    }
//Insert row
pub fn insert_row(
    &mut self,
    table_name: &str,
    values: Vec<Value>,
) {

    let table = match self.tables.get_mut(table_name) {
        Some(table) => table,
        None => {
            println!("Table '{}' does not exist.", table_name);
            return;
        }
    };

    // Check number of values
    if values.len() != table.columns.len() {
        println!(
            "Expected {} values but got {}.",
            table.columns.len(),
            values.len()
        );
        return;
    }

    // Check the types match the table schema
    for (value, column) in values.iter().zip(table.columns.iter()) {

        match (&column.data_type, value) {

            (DataType::Integer, Value::Integer(_)) => {}

            (DataType::Text, Value::Text(_)) => {}

            (DataType::Boolean, Value::Boolean(_)) => {}

            _ => {
                println!(
                    "Type mismatch for column '{}'.",
                    column.name
                );
                return;
            }
        }

    }

    // PRIMARY KEY & UNIQUE validation
    for (column_index, column) in table.columns.iter().enumerate() {

        if column.primary_key || column.unique {

            let new_value = &values[column_index];

            let existing_rows = self.read_rows_from_page(table_name);

            for existing_row in &existing_rows {

                if existing_row.values[column_index] == *new_value {

                    if column.primary_key {

                        println!(
                            "Error: Duplicate PRIMARY KEY '{}' in column '{}'.",
                            new_value,
                            column.name
                        );

                    } else {

                        println!(
                            "Error: Duplicate value '{}' in UNIQUE column '{}'.",
                            new_value,
                            column.name
                        );

                    }

                    return;

                }

            }

        }

    }

    // Store in memory (temporary until storage engine replaces table.rows)
    let row = Row {
        values,
    };
    
    println!("Row inserted.");
    
    let bytes = serde_json::to_vec(&row).unwrap();

    // Find this table's root page
    let root_page = self
        .catalog
        .get(table_name)
        .unwrap()
        .root_page;

    // Load the page
    let mut page = self.pager.load_page(root_page);

    // Let the page store the row
    if !page.insert_row(&bytes) {

        let new_page = self.pager.allocate_page();

        println!(
            "Allocated overflow page {}",
            new_page
        );

        return;

    }

    // Save page
    self.pager.save_page(&page);

    println!(
        "Row written to page {}",
        root_page
    );

    self.rebuild_indexes(table_name);

}

    pub fn delete_where(
        &mut self,
        table_name: &str,
        where_column: &str,
        where_value: &Value,
    ) {
        // Check for foreign key references before deleting
for (child_name, child_table) in &self.tables {

    if child_name == table_name {
        continue;
    }

    for (column_index, column) in child_table.columns.iter().enumerate() {

        if column.foreign_table.as_deref() == Some(table_name)
            && column.foreign_column.as_deref() == Some(where_column)
            && !column.cascade_delete
        {

            let referenced = child_table.rows.iter().any(|row| {
                row.values[column_index] == *where_value
            });

            if referenced {

                println!(
                    "Cannot delete: '{}' is referenced by table '{}'.",
                    where_value,
                    child_name
                );

                return;

            }

        }

    }

}
        // First borrow (only to delete rows)
        let deleted_values = {
            let table = match self.tables.get_mut(table_name) {
                Some(table) => table,
                None => {
                    println!("Table '{}' does not exist.", table_name);
                    return;
                }
            };
    
            let where_index = match table
                .columns
                .iter()
                .position(|c| c.name == where_column)
            {
                Some(index) => index,
                None => {
                    println!("Column '{}' does not exist.", where_column);
                    return;
                }
            };
    
            let mut deleted_values = Vec::new();
    
            for row in &table.rows {
                if row.values[where_index] == *where_value {
                    deleted_values.push(row.values[where_index].clone());
                }
            }
    
            let before = table.rows.len();
    
            table.rows.retain(|row| {
                row.values[where_index] != *where_value
            });
    
            let deleted = before - table.rows.len();
    
            println!("{} row(s) deleted.", deleted);
    
            deleted_values
        };
    
        // Borrow has ended here.
    
        self.rebuild_indexes(table_name);
    
        // Cascade delete
        let child_tables: Vec<String> = self.tables.keys().cloned().collect();
    
        for child_name in child_tables {
    
            if child_name == table_name {
                continue;
            }
    
            let child = self.tables.get_mut(&child_name).unwrap();
    
            for column_index in 0..child.columns.len() {
    
                let column = &child.columns[column_index];
    
                if column.cascade_delete
                    && column.foreign_table.as_deref() == Some(table_name)
                    && column.foreign_column.as_deref() == Some(where_column)
                {
    
                    child.rows.retain(|row| {
    
                        !deleted_values.contains(&row.values[column_index])
    
                    });
    
                }
    
            }
    
        }
    
        // Rebuild indexes again
        let table_names: Vec<String> = self.tables.keys().cloned().collect();
    
        for name in table_names {
            self.rebuild_indexes(&name);
        }
    }

//Read Rows from page
    pub fn read_rows_from_page(
        &mut self,
        table_name: &str,
    ) -> Vec<Row> {
    
        let root_page = self.catalog
            .get(table_name)
            .unwrap()
            .root_page;
    
        let page = self.pager.load_page(root_page);
    
        let mut rows = Vec::new();
    
        for bytes in page.get_rows() {
    
            rows.push(
                serde_json::from_slice(&bytes).unwrap()
            );
    
        }
    
        rows
    
    }


//  join
    pub fn join(
        &self,
        left_table: &str,
        right_table: &str,
        left_column: &str,
        right_column: &str,
    ) {
    
        let left = match self.tables.get(left_table) {
            Some(table) => table,
            None => {
                println!("Table '{}' does not exist.", left_table);
                return;
            }
        };
    
        let right = match self.tables.get(right_table) {
            Some(table) => table,
            None => {
                println!("Table '{}' does not exist.", right_table);
                return;
            }
        };
    
        let left_index = match left.columns.iter().position(|c| c.name == left_column) {
            Some(i) => i,
            None => {
                println!("Column '{}' not found.", left_column);
                return;
            }
        };
    
        let right_index = match right.columns.iter().position(|c| c.name == right_column) {
            Some(i) => i,
            None => {
                println!("Column '{}' not found.", right_column);
                return;
            }
        };
    
        // Print headers
        for column in &left.columns {
            print!("{:<15}", format!("{}.{}", left_table, column.name));
        }
    
        for column in &right.columns {
            print!("{:<15}", format!("{}.{}", right_table, column.name));
        }
    
        println!();
    
        // Print separator
        for _ in 0..(left.columns.len() + right.columns.len()) {
            print!("{:-<15}", "");
        }
    
        println!();
    
        // Nested-loop join
        for left_row in &left.rows {
    
            for right_row in &right.rows {
    
                if left_row.values[left_index] == right_row.values[right_index] {
    
                    for value in &left_row.values {
                        print!("{:<15}", value);
                    }
    
                    for value in &right_row.values {
                        print!("{:<15}", value);
                    }
    
                    println!();
    
                }
    
            }
    
        }
    
    }

    pub fn print_table(
        &mut self,
        table_name: &str,
        columns: Option<&[String]>,
    ) {
        let table = match self.tables.get(table_name) {
            Some(table) => table,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        // Read rows from storage
        let rows = self.read_rows_from_page(table_name);
    
        // Which columns should be displayed?
        let selected: Vec<usize> = match columns {
    
            Some(names) => {
    
                let mut indices = Vec::new();
    
                for name in names {
    
                    match table.columns.iter().position(|c| c.name == *name) {
    
                        Some(index) => indices.push(index),
    
                        None => {
                            println!("Column '{}' does not exist.", name);
                            return;
                        }
    
                    }
    
                }
    
                indices
    
            }
    
            None => (0..table.columns.len()).collect(),
    
        };
    
        // Calculate widths
        let mut widths = Vec::new();
    
        for &index in &selected {
    
            let mut width = table.columns[index].name.len();
    
            for row in &rows {
    
                let len = row.values[index].to_string().len();
    
                if len > width {
                    width = len;
                }
    
            }
    
            widths.push(width + 4);
    
        }
    
        // Print headers
        for (&index, width) in selected.iter().zip(widths.iter()) {
    
            print!(
                "{:<width$}",
                table.columns[index].name,
                width = *width
            );
    
        }
    
        println!();
    
        // Print separator
        for width in &widths {
    
            print!("{:-<width$}", "", width = *width);
    
        }
    
        println!();
    
        // Print rows
        for row in &rows {
    
            for (&index, width) in selected.iter().zip(widths.iter()) {
    
                let text = row.values[index].to_string();
    
                print!(
                    "{:<width$}",
                    text,
                    width = *width
                );
    
            }
    
            println!();
    
        }
    
    }
    
        // Select Function
        pub fn select_where(
            &self,
            table_name: &str,
            column_name: &str,
            operator: &str,
            value: &Value,
        ) {
            let table = match self.tables.get(table_name) {
                Some(table) => table,
                None => {
                    println!("Table '{}' does not exist.", table_name);
                    return;
                }
            };
        
            // Find the column
            let column_index = match table
                .columns
                .iter()
                .position(|c| c.name == column_name)
            {
                Some(index) => index,
                None => {
                    println!("Column '{}' does not exist.", column_name);
                    return;
                }
            };
        
            // Print table header
            for column in &table.columns {
                print!("{:<15}", column.name);
            }
            println!();
        
            for _ in &table.columns {
                print!("{:-<15}", "");
            }
            println!();
        
            // =====================================================
            // FAST PATH - Use the hash index if one exists
            // =====================================================
        
            if let Some(index) = table.indexes.get(column_name) {
        
                println!("Using HASH INDEX\n");
        
                if let Some(row_indexes) = index.get(&value.to_string()) {
        
                    for row_index in row_indexes {
        
                        let row = &table.rows[*row_index];
        
                        for cell in &row.values {
                            print!("{:<15}", cell);
                        }
        
                        println!();
                    }
        
                } else {
        
                    println!("No matching rows.");
        
                }
        
                return;
            }
        
            // =====================================================
            // SLOW PATH - Full table scan
            // =====================================================
        
            println!("Using FULL TABLE SCAN\n");
        
        
            for row in &table.rows {
        
                let current = &row.values[column_index];

let matches = match (current, value) {

    (Value::Integer(a), Value::Integer(b)) => {

        match operator {

            "="  => a == b,
            "!=" => a != b,
            ">"  => a > b,
            "<"  => a < b,
            ">=" => a >= b,
            "<=" => a <= b,

            _ => false,
        }

    }

    (Value::Text(a), Value::Text(b)) => {

        match operator {

            "="  => a == b,
            "!=" => a != b,
            ">"  => a > b,
            "<"  => a < b,
            ">=" => a >= b,
            "<=" => a <= b,

            _ => false,
        }

    }

    (Value::Boolean(a), Value::Boolean(b)) => {

        match operator {

            "="  => a == b,
            "!=" => a != b,

            _ => false,
        }

    }

    _ => false,
};

if matches {

    self.print_row(table, row);

} {
        
                    for cell in &row.values {
                        print!("{:<15}", cell);
                    }
        
                    println!();
                }
            }
        }
    pub fn update_where(
        &mut self,
        table_name: &str,
        target_column: &str,
        new_value: &Value,
        where_column: &str,
        where_value: &Value,
    ) {
        let table = match self.tables.get_mut(table_name) {
            Some(table) => table,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        // Find the column to update
        let target_index = match table
            .columns
            .iter()
            .position(|c| c.name == target_column)
        {
            Some(index) => index,
            None => {
                println!("Column '{}' does not exist.", target_column);
                return;
            }
        };
    
        // Find the WHERE column
        let where_index = match table
            .columns
            .iter()
            .position(|c| c.name == where_column)
        {
            Some(index) => index,
            None => {
                println!("Column '{}' does not exist.", where_column);
                return;
            }
        };
    
        // Parse the WHERE value
        // Values are already parsed by the parser.
let where_value = where_value.clone();
let new_value = new_value.clone();
        let mut updated = 0;
    
        for row in &mut table.rows {
            if row.values[where_index] == where_value {
                row.values[target_index] = new_value.clone();
                updated += 1;
            }
        }
        self.rebuild_indexes(table_name);
        println!("{} row(s) updated.", updated);
    }


    pub fn has_index(
        &self,
        table_name: &str,
        column_name: &str,
    ) -> bool {
    
        let table = match self.tables.get(table_name) {
            Some(table) => table,
            None => return false,
        };
    
        table.indexes.contains_key(column_name)
    }

    
        // Save Function
    pub fn save(&self, filename: &str) {
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if std::fs::write(filename, json).is_ok() {
                    println!("Database saved to '{}'.", filename);
                } else {
                    println!("Failed to write file.");
                }
            }
    
            Err(e) => {
                println!("Serialization error: {}", e);
            }
        }
    }
    //Rebuild Indexes
    pub fn rebuild_indexes(&mut self, table_name: &str) {

        let table = match self.tables.get_mut(table_name) {
            Some(t) => t,
            None => return,
        };
    
        table.indexes.clear();
    
        for (column_index, column) in table.columns.iter().enumerate() {

            // Only build indexes for PK or UNIQUE columns
            if !(column.primary_key || column.unique) {
                continue;
            }
        
            let mut map: HashMap<String, Vec<usize>> = HashMap::new();
        
            for (row_index, row) in table.rows.iter().enumerate() {
        
                let key = row.values[column_index].to_string();
        
                map.entry(key)
                    .or_insert(Vec::new())
                    .push(row_index);
            }
        
            table.indexes.insert(column.name.clone(), map);
        }
    }

    fn print_row(&self, table: &Table, row: &Row) {
        for value in &row.values {
            print!("| {:<15}", value.to_string());
        }
    
        println!("|");
    }

    //Order By
    pub fn select_order_by(
        &self,
        table_name: &str,
        column_name: &str,
    ) {
        let table = match self.tables.get(table_name) {
            Some(table) => table,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        let column_index = match table
            .columns
            .iter()
            .position(|c| c.name == column_name)
        {
            Some(index) => index,
            None => {
                println!("Column '{}' does not exist.", column_name);
                return;
            }
        };
    
        // Print header
        for column in &table.columns {
            print!("{:<15}", column.name);
        }
        println!();
    
        for _ in &table.columns {
            print!("{:-<15}", "");
        }
        println!();
    
        // Clone row references
        let mut rows: Vec<&Row> = table.rows.iter().collect();
    
        // Sort rows
        rows.sort_by(|a, b| {
    
            let left = &a.values[column_index];
            let right = &b.values[column_index];
    
            match (left, right) {
    
                (Value::Integer(x), Value::Integer(y)) => x.cmp(y),
    
                (Value::Text(x), Value::Text(y)) => x.cmp(y),
    
                (Value::Boolean(x), Value::Boolean(y)) => x.cmp(y),
    
                _ => std::cmp::Ordering::Equal,
            }
        });
    
        // Print sorted rows
        for row in rows {
    
            for value in &row.values {
                print!("{:<15}", value);
            }
    
            println!();
        }
    }
    //Limit
    pub fn select_limit(
        &self,
        table_name: &str,
        limit: usize,
    ) {
        let table = match self.tables.get(table_name) {
            Some(table) => table,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        // Print headers
        for column in &table.columns {
            print!("{:<15}", column.name);
        }
    
        println!();
    
        for _ in &table.columns {
            print!("{:-<15}", "");
        }
    
        println!();
    
        // Print only the first `limit` rows
        for row in table.rows.iter().take(limit) {
            for value in &row.values {
                print!("{:<15}", value);
            }
    
            println!();
        }
    }

    //Slect Using Index
    pub fn select_using_index(
        &self,
        table_name: &str,
        column_name: &str,
        value: &Value,
    ) {
    
        let table = match self.tables.get(table_name) {
            Some(table) => table,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        let index = match table.indexes.get(column_name) {
            Some(index) => index,
            None => {
                println!("No index exists on '{}'.", column_name);
                return;
            }
        };
    
        let key = value.to_string();
    
        let row_indexes = match index.get(&key) {
            Some(rows) => rows,
            None => {
                println!("No matching rows.");
                return;
            }
        };
    
        // Header
        for column in &table.columns {
            print!("{:<15}", column.name);
        }
        println!();
    
        for _ in &table.columns {
            print!("{:-<15}", "");
        }
        println!();
    
        // Rows
        for row_index in row_indexes {
    
            let row = &table.rows[*row_index];
    
            self.print_row(table, row);
    
        }
    }

//Debug first row
    pub fn debug_read_first_row(
        &mut self,
        table_name: &str,
    ) {
    
        let root_page = self.catalog
            .get(table_name)
            .unwrap()
            .root_page;
    
        let page = self.pager.load_page(root_page);
    
        let used = u64::from_le_bytes(
            page.data[0..8]
                .try_into()
                .unwrap()
        );
    
        if used <= 8 {
    
            println!("No rows stored.");
    
            return;
    
        }
    
        let bytes = &page.data[8..used as usize];
    
        let row = deserialize_row(bytes);
    
        println!("{:?}", row);
    
    }


    //Explain
    pub fn explain_select(
        &self,
        table_name: &str,
        column_name: &str,
        value: &Value,
    ) {
        let table = match self.tables.get(table_name) {
            Some(table) => table,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        println!();
        println!("========== QUERY PLAN ==========");
        println!();
    
        println!("Operation : SELECT");
        println!("Table     : {}", table_name);
        println!("Predicate : {} = {}", column_name, value);
    
        if table.indexes.contains_key(column_name) {
    
            let estimated_rows = table
                .indexes
                .get(column_name)
                .and_then(|idx| idx.get(&value.to_string()))
                .map(|rows| rows.len())
                .unwrap_or(0);
    
            println!("Access    : HASH INDEX");
            println!("Estimated Rows : {}", estimated_rows);
    
        } else {
    
            println!("Access    : FULL TABLE SCAN");
            println!("Estimated Rows : {}", table.rows.len());
    
        }
    
        println!();
    }


    pub fn count(
        &self,
        table_name: &str,
    ) {
        let table = match self.tables.get(table_name) {
            Some(table) => table,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        println!("Count: {}", table.rows.len());
    }
    
    pub fn sum(
        &self,
        table_name: &str,
        column_name: &str,
    ) {
        let table = match self.tables.get(table_name) {
            Some(t) => t,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        let column_index = match table
            .columns
            .iter()
            .position(|c| c.name == column_name)
        {
            Some(i) => i,
            None => {
                println!("Column '{}' not found.", column_name);
                return;
            }
        };
    
        let mut total = 0;
    
        for row in &table.rows {
            match &row.values[column_index] {
                Value::Integer(v) => total += *v,
                _ => {
                    println!("SUM only works on integer columns.");
                    return;
                }
            }
        }
    
        println!("Sum = {}", total);
    }
    
    pub fn avg(
        &self,
        table_name: &str,
        column_name: &str,
    ) {
        let table = match self.tables.get(table_name) {
            Some(t) => t,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        let column_index = match table
            .columns
            .iter()
            .position(|c| c.name == column_name)
        {
            Some(i) => i,
            None => {
                println!("Column '{}' not found.", column_name);
                return;
            }
        };
    
        let mut total = 0;
        let mut count = 0;
    
        for row in &table.rows {
            match &row.values[column_index] {
                Value::Integer(v) => {
                    total += *v;
                    count += 1;
                }
                _ => {
                    println!("AVG only works on integer columns.");
                    return;
                }
            }
        }
    
        if count == 0 {
            println!("Average = 0");
        } else {
            println!("Average = {}", total as f64 / count as f64);
        }
    }
    
    pub fn min(
        &self,
        table_name: &str,
        column_name: &str,
    ) {
        let table = match self.tables.get(table_name) {
            Some(t) => t,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        let column_index = match table.columns.iter().position(|c| c.name == column_name) {
            Some(i) => i,
            None => {
                println!("Column '{}' not found.", column_name);
                return;
            }
        };
    
        let mut minimum: Option<i64> = None;
    
        for row in &table.rows {
            match &row.values[column_index] {
                Value::Integer(v) => {
                    minimum = Some(match minimum {
                        Some(current) => current.min(*v),
                        None => *v,
                    });
                }
                _ => {
                    println!("MIN only works on integer columns.");
                    return;
                }
            }
        }
    
        match minimum {
            Some(v) => println!("Min = {}", v),
            None => println!("Table is empty."),
        }
    }
    
    pub fn max(
        &self,
        table_name: &str,
        column_name: &str,
    ) {
        let table = match self.tables.get(table_name) {
            Some(t) => t,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
    
        let column_index = match table.columns.iter().position(|c| c.name == column_name) {
            Some(i) => i,
            None => {
                println!("Column '{}' not found.", column_name);
                return;
            }
        };
    
        let mut maximum: Option<i64> = None;
    
        for row in &table.rows {
            match &row.values[column_index] {
                Value::Integer(v) => {
                    maximum = Some(match maximum {
                        Some(current) => current.max(*v),
                        None => *v,
                    });
                }
                _ => {
                    println!("MAX only works on integer columns.");
                    return;
                }
            }
        }
    
        match maximum {
            Some(v) => println!("Max = {}", v),
            None => println!("Table is empty."),
        }
    }
//Begin transaction
    pub fn begin_transaction(&mut self) {

        if self.transaction_backup.is_some() {
            println!("A transaction is already active.");
            return;
        }
    
        self.transaction_backup = Some(self.tables.clone());
    
        println!("Transaction started.");
    }
    
    //Commit Transaction
    pub fn commit_transaction(&mut self) {

        if self.transaction_backup.is_none() {
            println!("No active transaction.");
            return;
        }
    
        self.transaction_backup = None;
    
        println!("Transaction committed.");
    }
    
    //Rollback Transaction
    pub fn rollback_transaction(&mut self) {

        match self.transaction_backup.take() {
    
            Some(backup) => {
    
                self.tables = backup;
    
                println!("Transaction rolled back.");
    
            }
    
            None => {
    
                println!("No active transaction.");
    
            }
    
        }
    
    }


    pub fn find(
        &self,
        table_name: &str,
        conditions: &[Condition],
    ) {
        let table = match self.tables.get(table_name) {
            Some(t) => t,
            None => {
                println!("Table '{}' does not exist.", table_name);
                return;
            }
        };
        if conditions.len() == 1 {

            let condition = &conditions[0];
        
            if let Operator::Equal = condition.operator {
        
                if let Some(index) = table.indexes.get(&condition.column) {
        
                    let key = condition.value.to_string();
        
                    if let Some(rows) = index.get(&key) {
        
                        // Print headers
                        for column in &table.columns {
                            print!("{:<15}", column.name);
                        }
        
                        println!();
        
                        for _ in &table.columns {
                            print!("{:-<15}", "");
                        }
        
                        println!();
        
                        for &row_index in rows {
        
                            let row = &table.rows[row_index];
        
                            for value in &row.values {
        
                                print!("{:<15}", value);
        
                            }
        
                            println!();
        
                        }
        
                        return;
        
                    }
        
                }
        
            }
        
        }
    
        // Print headers
        for column in &table.columns {
            print!("{:<15}", column.name);
        }
    
        println!();
    
        for _ in &table.columns {
            print!("{:-<15}", "");
        }
    
        println!();
    
        // Check every row
        for row in &table.rows {
    
            if self.matches_conditions(
                table,
                row,
                conditions,
            ) {
    
                for value in &row.values {
                    print!("{:<15}", value);
                }
    
                println!();
    
            }
    
        }
    }


    fn matches_conditions(
        &self,
        table: &Table,
        row: &Row,
        conditions: &[Condition],
    ) -> bool {
    
        if conditions.is_empty() {
            return true;
        }
    
        // Evaluate the first condition
        let mut result = self.evaluate_condition(
            table,
            row,
            &conditions[0],
        );
    
        // Combine the remaining conditions
        for i in 1..conditions.len() {
    
            let comparison = self.evaluate_condition(
                table,
                row,
                &conditions[i],
            );
    
            match conditions[i - 1].logic {
    
                Some(Logic::And) => {
                    result = result && comparison;
                }
    
                Some(Logic::Or) => {
                    result = result || comparison;
                }
    
                None => {
                    result = comparison;
                }
    
            }
    
        }
    
        result
    }

    fn evaluate_condition(
        &self,
        table: &Table,
        row: &Row,
        condition: &Condition,
    ) -> bool {
    
        let column_index = match table
            .columns
            .iter()
            .position(|c| c.name == condition.column)
        {
            Some(i) => i,
            None => return false,
        };
    
        self.compare_values(
            &row.values[column_index],
            &condition.operator,
            &condition.value,
        )
    }



    fn compare_values(
        &self,
        left: &Value,
        operator: &Operator,
        right: &Value,
    ) -> bool {
    
        match (left, right) {
    
            (Value::Integer(a), Value::Integer(b)) => {
    
                match operator {
    
                    Operator::Equal => a == b,
    
                    Operator::NotEqual => a != b,
    
                    Operator::Greater => a > b,
    
                    Operator::Less => a < b,
    
                    Operator::GreaterEqual => a >= b,
    
                    Operator::LessEqual => a <= b,
    
                    Operator::Like => false,
    
                }
    
            }
    
            (Value::Text(a), Value::Text(b)) => {
    
                match operator {
    
                    Operator::Equal => a == b,
    
                    Operator::NotEqual => a != b,
    
                    Operator::Greater => a > b,
    
                    Operator::Less => a < b,
    
                    Operator::GreaterEqual => a >= b,
    
                    Operator::LessEqual => a <= b,
    
                    Operator::Like => {
    
                        if b.ends_with('*') {
    
                            let prefix = &b[..b.len() - 1];
    
                            a.starts_with(prefix)
    
                        } else {
    
                            a == b
    
                        }
    
                    }
    
                }
    
            }
    
            (Value::Boolean(a), Value::Boolean(b)) => {
    
                match operator {
    
                    Operator::Equal => a == b,
    
                    Operator::NotEqual => a != b,
    
                    Operator::Like => false,
    
                    _ => false,
    
                }
    
            }
    
            _ => false,
    
        }
    
    }
//Group by
pub fn group_by(
    &self,
    table_name: &str,
    group_column: &str,
    aggregate: &str,
    target_column: Option<&str>,
) {
    let table = match self.tables.get(table_name) {
        Some(t) => t,
        None => {
            println!("Table '{}' does not exist.", table_name);
            return;
        }
    };

    // Find the GROUP BY column
    let group_index = match table
        .columns
        .iter()
        .position(|c| c.name == group_column)
    {
        Some(i) => i,
        None => {
            println!("Column '{}' not found.", group_column);
            return;
        }
    };

    // Find the aggregation column (if there is one)
    let target_index = target_column.and_then(|name| {
        table.columns.iter().position(|c| c.name == name)
    });

    // Create the groups
    let mut groups: HashMap<String, Vec<&Row>> = HashMap::new();

    // Put every row into its group
    for row in &table.rows {

        let key = row.values[group_index].to_string();

        groups
            .entry(key)
            .or_insert(Vec::new())
            .push(row);
    }

    println!("{:<20}{}", group_column, aggregate);

for (group, rows) in groups {

    match aggregate {

        "COUNT" => {

            println!("{:<20}{}", group, rows.len());

        }

        "SUM" => {

            let index = match target_index {
                Some(i) => i,
                None => {
                    println!("SUM requires a column.");
                    return;
                }
            };

            let mut total = 0;

            for row in &rows {

                match &row.values[index] {

                    Value::Integer(v) => total += *v,

                    _ => {
                        println!("SUM only works on integer columns.");
                        return;
                    }

                }

            }

            println!("{:<20}{}", group, total);

        }

        "AVG" => {

            let index = match target_index {
                Some(i) => i,
                None => {
                    println!("AVG requires a column.");
                    return;
                }
            };

            let mut total = 0;

            for row in &rows {

                match &row.values[index] {

                    Value::Integer(v) => total += *v,

                    _ => {
                        println!("AVG only works on integer columns.");
                        return;
                    }

                }

            }

            let average = total as f64 / rows.len() as f64;

            println!("{:<20}{:.2}", group, average);

        }

        "MIN" => {

            let index = match target_index {
                Some(i) => i,
                None => {
                    println!("MIN requires a column.");
                    return;
                }
            };

            let mut minimum: Option<i64> = None;

            for row in &rows {

                match &row.values[index] {

                    Value::Integer(v) => {

                        minimum = Some(match minimum {
                            Some(current) => current.min(*v),
                            None => *v,
                        });

                    }

                    _ => {
                        println!("MIN only works on integer columns.");
                        return;
                    }

                }

            }

            println!("{:<20}{}", group, minimum.unwrap());

        }

        "MAX" => {

            let index = match target_index {
                Some(i) => i,
                None => {
                    println!("MAX requires a column.");
                    return;
                }
            };

            let mut maximum: Option<i64> = None;

            for row in &rows {

                match &row.values[index] {

                    Value::Integer(v) => {

                        maximum = Some(match maximum {
                            Some(current) => current.max(*v),
                            None => *v,
                        });

                    }

                    _ => {
                        println!("MAX only works on integer columns.");
                        return;
                    }

                }

            }

            println!("{:<20}{}", group, maximum.unwrap());

        }

        _ => {

            println!("Unknown aggregate.");

        }

    }

}
}

        // Load Function
    pub fn load(&mut self, filename: &str) {
        let contents = match std::fs::read_to_string(filename) {
            Ok(text) => text,
            Err(_) => {
                println!("Could not open '{}'.", filename);
                return;
            }
        };
    
        match serde_json::from_str::<Database>(&contents) {
            Ok(db) => {
                *self = db;
                self.pager = Pager::new("quartz.db");
                println!("Database loaded.");
            }
    
            Err(e) => {
                println!("Failed to load database: {}", e);
            }
        }
    }
}