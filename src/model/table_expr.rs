use crate::model::column_rule::{ColumnRule, NonNull, PrimaryKey};
use std::fmt::{Debug, Display};
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
    #[cfg(test)]
    pub fn new(table_name: &str, schema_name: Option<&str>, alias: Option<&str>) -> Self {
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

impl Display for TableRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self.schema_name {
            Some(schema_name) => format!("{}.{}", schema_name, self.table_name),
            None => self.table_name.clone(),
        };
        write!(f, "{}", str)
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
    pub fn new(name: String, data_type: DataType, not_null: bool, primary_key: bool) -> Self {
        let mut rules = Vec::new();
        if not_null {
            rules.push(ColumnRule::NonNull(NonNull::new(None, None, None)));
        }
        if primary_key {
            rules.push(ColumnRule::PrimaryKey(PrimaryKey::new(None, None)));
        }

        Self {
            name,
            data_type,
            not_null,
            primary_key,
            rules,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DataType {
    pub name: String,
    pub size: Option<[Option<u32>; 2]>,
}

impl DataType {

    pub fn new(name: &str, size1: Option<u32>, size2: Option<u32>) -> Self {
        let mut size: Option<[Option<u32>; 2]> = None;

        if size1.is_some() {
            size = Some([size1, size2]);
        }

        Self {
            name: name.to_owned(),
            size,
        }
    }
}
