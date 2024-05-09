use std::fmt::Debug;
use std::str::FromStr;

pub struct TableDef {
    pub table_ref: String,
    pub columns: Vec<Box<ColumnDef>>,
    // table_level_rules: Vec<TableLevelRule>,
}

pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub not_null: bool,
    pub primary_key: bool,
}

impl Default for ColumnDef {
    fn default() -> Self {
        Self {
            name: String::new(),
            data_type: DataType::default(),
            not_null: false,
            primary_key: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataType {
    pub name: String,
    pub size : Option<[Option<u32>;2]>
}

impl Default for DataType {
    fn default() -> Self {
        Self {
            name: String::new(),
            size: None
        }
    }
}

impl DataType {
    pub fn f_name(name: &str) -> Self {
        Self {
            name: String::from(name),
            size: None
        }
    }

    pub fn f_name_1_size(name: &str, size1: u32) -> Self {
        Self {
            name: String::from(name),
            size: Some([Some(size1), None])
        }
    }
}

pub struct TableLevelRule {
    pub name: String,
}
