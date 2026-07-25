use crate::database::Row;

pub fn serialize_row(row: &Row) -> Vec<u8> {
    serde_json::to_vec(row).unwrap()
}

pub fn deserialize_row(bytes: &[u8]) -> Row {
    serde_json::from_slice(bytes).unwrap()
}