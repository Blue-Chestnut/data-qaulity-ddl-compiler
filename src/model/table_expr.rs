use crate::model::column_rule::{ColumnRule, IsType, NonNull, Uniqueness};
use crate::model::data_class::DataClass;
use lalrpop_util::lalrpop_mod;
use std::fmt::{Debug, Display};
use std::str::FromStr;

use super::rule_filter::filter::ColumnRuleFilter;

lalrpop_mod!(pub data_class, "/parser/data_class_parsing.rs");

#[derive(Clone, Debug, PartialEq)]
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
    pub rules: Vec<ColumnRuleFilter>,
}

impl ColumnDef {
    pub fn new(name: String, data_type: DataType, not_null: bool, primary_key: bool) -> Self {
        let mut rules = Vec::new();
        if not_null {
            rules.push(ColumnRule::NonNull(NonNull::new(None, None, None)));
        }
        if primary_key {
            rules.push(ColumnRule::Uniqueness(Uniqueness::new(None, None)));
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
            rules: vec![ColumnRuleFilter::empty_fr_rules(rules)],
        }
    }

    // pub fn new_with_rules(
    //     name: String,
    //     data_type: DataType,
    //     not_null: bool,
    //     primary_key: bool,
    //     rules: Vec<ColumnRule>,
    // ) -> Self {
    //     let mut column = Self::new(name, data_type, not_null, primary_key);
    //     column.rules.push(ColumnRuleFilter::new(None, rules));
    //     column
    // }

    pub fn new_with_rules_and_filter(
        name: String,
        data_type: DataType,
        not_null: bool,
        primary_key: bool,
        rules: Vec<ColumnRuleFilter>,
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

#[cfg(test)]
pub mod test {
    use crate::model::{
        column_rule::{IsType, NonNull, Uniqueness},
        rule_filter::filter::ColumnRuleFilter,
        table_expr::{ColumnDef, ColumnRule, DataType},
    };
    use rstest::rstest;

    #[rstest]
    #[case(ColumnDef {name: "Example".to_owned(), data_type: DataType::new("INT", Some(3), None), not_null: true, primary_key: true, rules: 
    vec![ColumnRuleFilter::empty_fr_rules(vec![
        ColumnRule::NonNull(NonNull::new(None, None, None)),
        ColumnRule::Uniqueness(Uniqueness::new(None, None)),
        ColumnRule::IsType(IsType {name: "".to_owned(), data_type: DataType::new("Int", Some(3), None), ..Default::default()}),
    ])]}, "Example".to_owned(), true, true)]
    #[case(ColumnDef {name: "Example".to_owned(), data_type: DataType::new("INT", Some(3), None), not_null: true, primary_key: false, rules: 
    vec![ColumnRuleFilter::empty_fr_rules(vec![
        ColumnRule::NonNull(NonNull::new(None, None, None)),
        ColumnRule::IsType(IsType {name: "".to_owned(), data_type: DataType::new("Int", Some(3), None), ..Default::default()}),
    ])]}, "Example".to_owned(), true, false)]
    #[case(ColumnDef {name: "Example".to_owned(), data_type: DataType::new("INT", Some(3), None), not_null: false, primary_key: false, rules: 
    vec![ColumnRuleFilter::empty_fr_rules(vec![
        ColumnRule::IsType(IsType {name: "".to_owned(), data_type: DataType::new("Int", Some(3), None), ..Default::default()}),
    ])]}, "Example".to_owned(), false, false)]
    fn test_col_def_init(
        #[case] desired_col_def: ColumnDef,
        #[case] name: String,
        #[case] not_null: bool,
        #[case] primary_key: bool,
    ) {
        let col_def = ColumnDef::new(
            name,
            desired_col_def.data_type.clone(),
            not_null,
            primary_key,
        );
        assert_eq!(desired_col_def, col_def);
    }
}
