use crate::compiler::dqdl;
use crate::compiler::pydeequ;
use crate::compiler::CompilationTarget;
use clap::Parser;
use std::fs::{read_to_string, File};
use std::io::Write;

mod compiler;
mod model;
mod parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Compilation Target
    #[arg(short, long, value_enum)]
    target: CompilationTarget,

    /// Input string for compilation, will prefer input file over input string
    #[arg(short = 's', long, default_value_t = String::new())]
    input_string: String,

    /// Input file path for compilation
    #[arg(short = 'f', long, default_value_t = String::new())]
    input_file: String,

    /// Output file path for compilation
    #[arg(short = 'o', long, default_value_t = String::new())]
    output_file: String,
}

impl Args {
    fn get_input_string(&self) -> String {
        if !self.input_file.is_empty() {
            let contents = read_to_string(self.input_file.as_str()).unwrap();
            return contents;
        }

        if !self.input_string.is_empty() {
            return self.input_string.clone();
        }

        panic!("input is not defined")
    }
}

pub fn main() {
    // println!("{:?}", "Create table if not exists Test {".to_owned().len());
    // println!("{:?}", "    Id Varchar(10) {".to_owned().len());
    // println!("{:?}", "        -LIKE \"%test%\",".to_owned().len());
    // println!("{:?}", "        -REGEX \"[0-9]*test[0-9]*\",.".to_owned().len());
    // return;
    let args = Args::parse();

    println!("{:?}", args);

    let input_string = args.get_input_string();

    let table_def = parser::parse(input_string.as_str());

    if table_def.is_none() {
        panic!("Failed to parse input");
    }
    let table_def = *table_def.unwrap();

    let compiled: String = match args.target {
        CompilationTarget::PyDeequ => pydeequ::compile(table_def),
        CompilationTarget::Dqdl => dqdl::compile(table_def),
        _ => unimplemented!("Cannot compile to target: {:?}", args.target),
    };

    println!("{}", compiled);

    if !args.output_file.is_empty() {
        let mut output = File::create(args.output_file).expect("Couldn't find file");
        output
            .write_all(compiled.as_bytes())
            .expect("Couldn't write to file");
    }
    // let parsed_result = table::CreateTableExprParser::new().parse("create table if not exists\n Test \n{\nId FLOAT\n { -LIKE \"%test%\", -REGEX \"[0-9]*test[0-9]*\", -CONTAINS \"test\" },Price FLOAT\n }\n;");
    // println!("{:?}", parsed_result.as_ref().err());
    // if parsed_result.as_ref().is_ok() {
    //     let table_def = parsed_result.unwrap();
    //
    //     let table_check_python_code = compile(table_def);
    //     println!("{}", table_check_python_code);
    //
    //     let path = "../data-quality-ddl-compiler-python-test/ddl_dq_example/id_check.py";
    //     let mut output = File::create(path)?;
    //     write!(output, "{}", table_check_python_code)
    // } else {
    //     Ok(())
    // }
}
