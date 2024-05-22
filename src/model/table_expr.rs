use crate::model::column_rule::{ColumnRule, IsType, NonNull, PrimaryKey};
use crate::model::data_class::DataClass;
use lalrpop_util::lalrpop_mod;
use std::fmt::{Debug, Display};
use std::str::FromStr;

lalrpop_mod!(pub data_class, "/model/data_class.rs");

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

        if !data_type.class.is_date_like() {
            rules.push(ColumnRule::IsType(IsType::new(
                None,
                data_type.clone(),
                None,
            )))
        }

        Self {
            name,
            data_type,
            not_null,
            primary_key,
            rules,
        }
    }

    pub fn new_with_rules(
        name: String,
        data_type: DataType,
        not_null: bool,
        primary_key: bool,
        rules: Vec<ColumnRule>,
    ) -> Self {
        let mut column = Self::new(name, data_type, not_null, primary_key);
        column.rules.extend(rules);
        column
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DataType {
    pub class: DataClass,
    pub size: Option<[Option<u32>; 2]>,
}

impl DataType {
    pub fn new(name: &str, size1: Option<u32>, size2: Option<u32>) -> Self {
        if size1.is_some() && size2.is_some() {
            return DataType::from_str(
                format!("{} ({}, {})", name, size1.unwrap(), size2.unwrap()).as_str(),
            )
            .unwrap();
        }
        if size1.is_some() {
            return DataType::from_str(format!("{} ({})", name, size1.unwrap()).as_str()).unwrap();
        }
        DataType::from_str(name).unwrap()
    }
}

impl FromStr for DataType {
    type Err = ();

    fn from_str(name: &str) -> Result<Self, ()> {
        Ok(data_class::DataTypeExprParser::new().parse(name).unwrap())
    }
}
