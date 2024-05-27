use std::error::Error;

use crate::model::rule_traits::ValidColumnRule;
use crate::model::table_expr::TableDef;
use lalrpop_util::lalrpop_mod;

pub mod create_table_tests;

pub mod lines;

lalrpop_mod!(pub table, "/parser/create_table.rs");

pub fn parse(input_string: &str) -> Option<Box<TableDef>> {
    let table_def = table::CreateTableExprParser::new().parse(input_string);
    if table_def.is_err() {
        let err = table_def.unwrap_err();
        println!("{:?}", err.source());
        return None;
    }
    let table = table_def.unwrap();

    for column in &table.columns {
        for rule in &column.rules {
            let result = rule.validate_col_type(column);
            if result.is_err() {
                panic!("{:?}", result.unwrap_err());
            }
        }
    }

    Some(table)
}
