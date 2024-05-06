use lalrpop_util::lalrpop_mod;

mod model;
mod parser;

lalrpop_mod!(pub table, "/parser/create_table.rs");

pub fn main() {
    let parsed_result =
        crate::parser::create_table_tests::table::ColumnDefExprParser::new().parse("Id INT");
    println!("{:?}", parsed_result.as_ref().err());
    if (parsed_result.as_ref().is_ok()) {
        let column_def = parsed_result.unwrap();
        println!("{:?} {:?}", column_def.name, column_def.data_type);
    }
    // println!("{:?}", parsed_result.);
}
