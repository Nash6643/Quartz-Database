use std::collections::HashMap;
use serde::{Serialize, Deserialize};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub root_page: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Catalog {
    pub tables: HashMap<String, TableInfo>,
}

impl Catalog {

    pub fn new() -> Self {

        Self {
            tables: HashMap::new(),
        }

    }

    pub fn register_table(
        &mut self,
        name: String,
        root_page: u64,
    ) {

        self.tables.insert(
            name.clone(),
            TableInfo {
                name,
                root_page,
            },
        );

    }

    pub fn get(
        &self,
        name: &str,
    ) -> Option<&TableInfo> {

        self.tables.get(name)

    }

}