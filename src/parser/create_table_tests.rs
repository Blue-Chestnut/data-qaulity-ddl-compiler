use lalrpop_util::lalrpop_mod;
use rstest::rstest;

lalrpop_mod!(pub table, "/parser/create_table.rs");

#[cfg(test)]
pub mod create_table_tests {
    use super::*;
    use crate::model::table_expr::ColumnDef;

    #[rstest]
    #[case("CREATE TABLE IF NOT EXISTS Inventory {Id INT,Title VARCHAR,  };", "Inventory",
    vec![
        ColumnDef {name: String::from("Id"), data_type: String::from("INT")},
        ColumnDef {name: String::from("Title"), data_type: String::from("VARCHAR")},
    ])]
    #[case("CREATE TABLE IF NOT EXISTS\n Test {Id FLOAT};", "Test",
    vec![
    ColumnDef {name: String::from("Id"), data_type: String::from("FLOAT")},
    ])]
    #[case(" CREATE TABLE IF NOT EXISTS\n Test \n{\nId FLOAT\n,}\n;\n", "Test",
    vec![
    ColumnDef {name: String::from("Id"), data_type: String::from("FLOAT")},
    ])]
    #[case(" CREATE TABLE IF NOT EXISTS\n Test33 \n{\nId FLOAT,\nPrice FLOAT,\nNotes TEXT\n}\n;\n",
    "Test33",
    vec![
    ColumnDef {name: String::from("Id"), data_type: String::from("FLOAT")},
    ColumnDef {name: String::from("Price"), data_type: String::from("FLOAT")},
    ColumnDef {name: String::from("Notes"), data_type: String::from("TEXT")},
    ])]
    fn test_create_table_success(
        #[case] input_value: &str,
        #[case] table_ref: &str,
        #[case] cols: Vec<ColumnDef>,
    ) {
        let parsed_result = table::CreateTableExprParser::new().parse(input_value);
        let parsed_result_ref = parsed_result.as_ref();

        assert!(parsed_result_ref.is_ok(), "{:?}", parsed_result_ref.err());

        let column_def = parsed_result_ref.unwrap().as_ref();

        assert_eq!(column_def.table_ref, table_ref);
        assert_eq!(column_def.columns.len(), cols.len());

        for (i, column) in column_def.columns.iter().enumerate() {
            let desired_col: &ColumnDef = &cols[i];
            assert_eq!(column.data_type, desired_col.data_type);
            assert_eq!(column.name, desired_col.name);
        }
    }

    #[rstest]
    #[case("CREATE TABLE IF NOT EXISTS Inventory {Id INT,Title VARCHAR,  }")]
    #[case("CREATE TABLE IF NOT EXISTS Inventory {Id INT Title VARCHAR,  };")]
    #[case("CREATE TABLE IF NOT EXISTS {Id INT,Title VARCHAR,  };")]
    fn test_create_table_failure(#[case] input_value: &str) {
        assert!(table::CreateTableExprParser::new()
            .parse(input_value)
            .is_err());
    }

    #[rstest]
    #[case("Id INT", "Id", "INT")]
    #[case("Id  INT ", "Id", "INT")]
    #[case(" Id INT", "Id", "INT")]
    #[case(" Id33 INT", "Id33", "INT")]
    fn test_column_def_success(
        #[case] input_value: &str,
        #[case] name: &str,
        #[case] date_type: &str,
    ) {
        let parsed_result = table::ColumnDefExprParser::new().parse(input_value);
        let parsed_result_ref = parsed_result.as_ref();

        assert!(parsed_result_ref.is_ok(), "{:?}", parsed_result_ref.err());

        let column_def = parsed_result_ref.unwrap().as_ref();

        assert_eq!(column_def.data_type, date_type);
        assert_eq!(column_def.name, name);
    }

    #[rstest]
    #[case("'22")]
    #[case("'22''")]
    #[case("3Id INT")]
    #[case("Id, INT")]
    fn test_column_def_failure(#[case] input_value: &str) {
        assert!(table::ColumnDefExprParser::new()
            .parse(input_value)
            .is_err());
    }
}
