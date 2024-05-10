use std::fmt::Debug;
use std::str::FromStr;

pub struct TableDef {
    pub table_ref: TableRef,
    pub columns: Vec<Box<ColumnDef>>,
    // table_level_rules: Vec<TableLevelRule>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableRef {
    pub table_name: String,
    pub schema_name: Option<String>,
    pub alias: Option<String>,
}

impl TableRef {
    pub fn from_str(table_name: &str, schema_name: Option<&str>, alias: Option<&str>) -> Self {
        Self {
            table_name: String::from(table_name),
            schema_name: schema_name.map(String::from),
            alias: alias.map(String::from),
        }
    }
}

impl FromStr for TableRef {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let table_name = parts.next().unwrap();
        let schema_name = parts.next();
        Ok(Self {
            table_name: String::from(table_name),
            schema_name: schema_name.map(String::from),
            alias: None,
        })
    }
}

impl Default for TableRef {
    fn default() -> Self {
        Self {
            table_name: String::new(),
            schema_name: None,
            alias: None,
        }
    }
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
    pub size: Option<[Option<u32>; 2]>,
}

impl Default for DataType {
    fn default() -> Self {
        Self {
            name: String::new(),
            size: None,
        }
    }
}

impl DataType {
    pub fn f_name(name: &str) -> Self {
        Self {
            name: String::from(name),
            size: None,
        }
    }

    pub fn f_name_1_size(name: &str, size1: u32) -> Self {
        Self {
            name: String::from(name),
            size: Some([Some(size1), None]),
        }
    }
}

pub struct TableLevelRule {
    pub name: String,
}
