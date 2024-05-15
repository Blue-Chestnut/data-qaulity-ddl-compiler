use crate::compiler::pydeequ::compile;
use lalrpop_util::lalrpop_mod;
use std::fs::File;
use std::io::Write;

mod compiler;
mod model;
mod parser;

lalrpop_mod!(pub table, "/parser/create_table.rs");

pub fn main() -> std::io::Result<()> {
    let parsed_result = table::CreateTableExprParser::new().parse("create table if not exists\n Test \n{\nId FLOAT\n { -LIKE \"%test%\", -REGEX \"[0-9]*test[0-9]*\", -CONTAINS \"test\" },Price FLOAT\n }\n;");
    println!("{:?}", parsed_result.as_ref().err());
    if parsed_result.as_ref().is_ok() {
        let table_def = parsed_result.unwrap();

        let table_check_python_code = compile(table_def);
        println!("{}", table_check_python_code);

        let path = "../data-quality-ddl-compiler-python-test/ddl_dq_example/id_check.py";
        let mut output = File::create(path)?;
        write!(output, "{}", table_check_python_code)
    } else {
        Ok(())
    }
    // println!("{:?}", parsed_result.);
}
