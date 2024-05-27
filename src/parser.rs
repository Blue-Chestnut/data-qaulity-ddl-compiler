use crate::model::table_expr::TableDef;
use crate::parser::error_utils::gen_syntax_error_message;
use crate::{model::rule_traits::ValidColumnRule, parser::lines::Line};
use lalrpop_util::{lalrpop_mod, ParseError};

pub mod create_table_tests;

mod error_utils;
pub mod lines;

lalrpop_mod!(pub table, "/parser/create_table.rs");

pub fn parse(input_string: &str) -> Option<Box<TableDef>> {
    let table_def = table::CreateTableExprParser::new().parse(input_string);
    if table_def.is_err() {
        let lines = Line::from_string(input_string.to_owned());

        let err = table_def.unwrap_err();
        match err {
            ParseError::InvalidToken { location } => {
                let line_number = Line::get_line(location as u32, &lines);
                let current_line = lines.get(line_number);
                panic!(
                    "SyntaxError in line {}: Invalid token at {}",
                    line_number,
                    location - current_line.unwrap().start as usize
                );
            }
            ParseError::UnrecognizedToken { token, expected } => {
                let message = gen_syntax_error_message(token, lines, expected);

                panic!("{}", message);
            }
            // ParseError::ExtraToken { token } => {
            //     panic!("Extra token {}", token);
            // }
            // ParseError::UnrecognizedEof { location, expected } => {
            //     panic!("Unrecognized EOF at {} expected {}", location, expected);
            // }
            _ => panic!("{:?}", err),
        }
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

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use rstest::rstest;

    #[test]
    fn test_parse() {
        let table = parse("CREATE TABLE test {id INT(3), name VARCHAR(255)};");
        assert!(table.is_some());
    }

    #[rstest]
    #[case("CREATE TABLE test {id INT(3)., name VARCHAR(255)};")]
    #[should_panic]
    fn test_parse_error(#[case] input_string: &str) {
        parse(input_string).unwrap();
    }
}
