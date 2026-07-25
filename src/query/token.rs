#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Make,
    Add,
    Show,
    Find,
    Change,
    Remove,
    Plan,
    Save,
    Load,
    Help,
    Exit,
    If,

    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    Join,
    On,

    And,
    Or,

    In,
    Like,
    Projection,

    Begin,
    Commit,
    Rollback,

    Count,
    Sum,
    Avg,
    Min,
    Max,
    Between,

    Group,

    Identifier(String),
    Integer(i64),
    Text(String),
}