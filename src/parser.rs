use crate::model::table_expr::TableDef;
use lalrpop_util::lalrpop_mod;

pub mod create_table_tests;

lalrpop_mod!(pub table, "/parser/create_table.rs");

pub fn parse(input_string: &str) -> Option<Box<TableDef>> {
    let table_def = table::CreateTableExprParser::new().parse(input_string);
    if table_def.is_err() {
        return None;
    }
    Some(table_def.unwrap())
}
