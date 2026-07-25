use crate::database::Value;

#[derive(Debug)]
pub enum Operator {
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Like,
}

#[derive(Debug)]
pub enum Logic {
    And,
    Or,
}

#[derive(Debug)]
pub struct Condition {
    pub column: String,
    pub operator: Operator,
    pub value: Value,
    pub logic: Option<Logic>,
}


#[derive(Debug)]
pub enum Query {

    Make {
        table: String,
        columns: Vec<String>,
    },

    Group {
        table: String,
        group_column: String,
        aggregate: String,
        target_column: Option<String>,
    },

    Add {
        table: String,
        values: Vec<Value>,
    },

    Show {
        table: String,
        columns: Option<Vec<String>>,
    },

    Find {
        table: String,
        conditions: Vec<Condition>,
    },

    Change {
        table: String,
        column: String,
        value: Value,
        where_column: String,
        where_value: Value,
    },

    Count {
        table: String,
    },
    
    Sum {
        table: String,
        column: String,
    },
    
    Avg {
        table: String,
        column: String,
    },
    
    Min {
        table: String,
        column: String,
    },
    
    Max {
        table: String,
        column: String,
    },

    Remove {
        table: String,
        column: String,
        value: Value,
    },

    Plan {
        table: String,
        column: String,
        value: Value,
    },

    Save {
        filename: String,
    },

    Load {
        filename: String,
    },

    Join {
        left_table: String,
        right_table: String,
        left_column: String,
        right_column: String,
    },
    Begin,

    Commit,

    Rollback,
    
    Help,

    Exit,
}