use crate::model::rule_filter::combine_itentical_filters;
use crate::model::rule_filter::filter::ColumnRuleFilter;
use crate::model::rule_traits::ValidColumnRule;
use crate::model::table_expr::{ColumnDef, TableDef};
use crate::parser::error_utils::{
    gen_syntax_error_message, gen_unknown_token_error_message, DDLxParseError,
};
use crate::parser::lines::Line;
use lalrpop_util::{lalrpop_mod, ParseError};

pub mod create_table_tests;
pub mod data_class_tests;
pub mod error_utils;
pub mod lines;
pub mod rule_filter_tests;

lalrpop_mod!(pub table, "/parser/create_table.rs");

pub fn parse(input_string: &str) -> Result<Box<TableDef>, DDLxParseError> {
    let table_def = table::CreateTableExprParser::new().parse(input_string);
    if table_def.is_err() {
        let lines = Line::from_string(input_string.to_owned());

        let err = table_def.unwrap_err();
        match err {
            ParseError::InvalidToken { location } => {
                let message = gen_unknown_token_error_message(location, lines);

                return Err(DDLxParseError::UnknownToken(message));
            }
            ParseError::UnrecognizedToken { token, expected } => {
                let message = gen_syntax_error_message(token, lines, expected);

                return Err(DDLxParseError::SyntaxError(message));
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
    let mut table = table_def.unwrap();

    let mut columns: Vec<ColumnDef> = vec![];

    for column in &table.columns {
        let mut parsed_filters: Vec<ColumnRuleFilter> = vec![];

        for rule in &column.rules {
            let filter_result = rule.parse();
            if filter_result.is_err() {
                return Err(filter_result.unwrap_err());
            }

            let optimized_filter = filter_result.unwrap();
            parsed_filters.push(optimized_filter.clone());

            let result = optimized_filter.validate_col_type(column);
            if result.is_err() {
                return Err(DDLxParseError::ColumnValidationError(format!(
                    "{:?}",
                    result.unwrap_err()
                )));
            }
        }

        parsed_filters = combine_itentical_filters(parsed_filters);

        columns.push(ColumnDef {
            name: column.name.clone(),
            data_type: column.data_type.clone(),
            primary_key: column.primary_key,
            not_null: column.not_null,
            rules: parsed_filters,
        })
    }

    Ok(Box::new(TableDef {
        table_ref: table.table_ref.to_owned(),
        columns,
    }))
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;
    use rstest::rstest;

    #[test]
    fn test_parse() {
        let table = parse("CREATE TABLE test {id INT(3), name VARCHAR(255) {-not_empty}};");
        assert!(table.is_ok());
    }

    #[rstest]
    #[case("CREATE TABLE test {id INT(3)., name VARCHAR(255)};")]
    fn test_parse_error(#[case] input_string: &str) {
        parse(input_string).is_err();
    }
}
