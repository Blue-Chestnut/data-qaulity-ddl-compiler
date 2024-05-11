use crate::model::column_rule::ColumnRule;
use std::fmt::Debug;
use std::str::FromStr;

pub struct TableDef {
    pub table_ref: TableRef,
    pub columns: Vec<ColumnDef>,
    // table_level_rules: Vec<TableLevelRule>,
}

#[derive(Clone, Debug, PartialEq, Default)]
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub not_null: bool,
    pub primary_key: bool,
    pub rules: Vec<ColumnRule>,
}

impl ColumnDef {
    pub fn add_rules(&mut self, rules: Vec<ColumnRule>) -> &mut Self {
        self.rules.extend(rules);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DataType {
    pub name: String,
    pub size: Option<[Option<u32>; 2]>,
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
