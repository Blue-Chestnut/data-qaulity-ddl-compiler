use lalrpop_util::lalrpop_mod;
use rstest::rstest;

lalrpop_mod!(pub table, "/parser/create_table.rs");

#[cfg(test)]
pub mod create_table_tests {
    use super::*;
    use crate::model::table_expr::ColumnDef;
    use crate::model::table_expr::DataType;

    #[rstest]
    #[case("CREATE TABLE IF NOT EXISTS Inventory {Id INT,Title VARCHAR,  };", "Inventory",
    vec![
        ColumnDef {name: String::from("Id"), data_type: DataType::f_name("INT"), ..Default::default()},
        ColumnDef {name: String::from("Title"), data_type: DataType::f_name("VARCHAR"), ..Default::default()},
    ])]
    #[case("CREATE TABLE IF NOT EXISTS\n Test {Id FLOAT};", "Test",
    vec![
    ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
    ])]
    #[case(" CREATE TABLE IF NOT EXISTS\n Test \n{\nId FLOAT\n,}\n;\n", "Test",
    vec![
    ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
    ])]
    #[case(" create table if not exists\n Test \n{\nId FLOAT\n,}\n;\n", "Test",
    vec![
    ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
    ])]
    #[case(" CREATE TABLE IF NOT EXISTS\n Test33 \n{\nId FLOAT,\nPrice FLOAT,\nNotes TEXT\n}\n;\n",
    "Test33",
    vec![
    ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
    ColumnDef {name: String::from("Price"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
    ColumnDef {name: String::from("Notes"), data_type: DataType::f_name("TEXT"), ..Default::default()},
    ])]
    #[case(" create table\n Test33 \n{\nId FLOAT PRIMARY KEY,\nPrice FLOAT,\nNotes TEXT not null\n}\n;\n",
    "Test33",
    vec![
    ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), primary_key: true, not_null: true},
    ColumnDef {name: String::from("Price"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
    ColumnDef {name: String::from("Notes"), data_type: DataType::f_name("TEXT"), not_null: true,..Default::default()},
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
    #[case("Id INT", "Id", DataType::f_name("INT"), false, false)]
    #[case("Id  INT ", "Id", DataType::f_name("INT"), false, false)]
    #[case(" Id INT", "Id", DataType::f_name("INT"), false, false)]
    #[case(" Id33 INT", "Id33", DataType::f_name("INT"), false, false)]
    #[case(" Id33 INT PRIMARY KEY", "Id33", DataType::f_name("INT"), true, true)]
    #[case(" Id33 INT primaRY KeY", "Id33", DataType::f_name("INT"), true, true)]
    #[case(" Id33 TEXT NOT NULL", "Id33", DataType::f_name("TEXT"), true, false)]
    #[case(" Id33 DOUBLE not null", "Id33", DataType::f_name("DOUBLE"), true, false)]
    #[case(" Id33 DOUBLE not NuLL", "Id33", DataType::f_name("DOUBLE"), true, false)]
    #[case(" Id33 DOUBLE(30) not NuLL", "Id33", DataType::f_name_1_size("DOUBLE", 30), true, false)]
    #[case(" Id33 DOUBLE(30,5) not NuLL", "Id33", DataType {name: String::from("DOUBLE"), size: Some([Some(30), Some(5)])}, true, false)]
    fn test_column_def_success(
        #[case] input_value: &str,
        #[case] name: &str,
        #[case] date_type: DataType,
        #[case] not_null: bool,
        #[case] primary_key: bool,
    ) {
        let parsed_result = table::ColumnDefExprParser::new().parse(input_value);
        let parsed_result_ref = parsed_result.as_ref();

        assert!(parsed_result_ref.is_ok(), "{:?}", parsed_result_ref.err());

        let column_def = parsed_result_ref.unwrap().as_ref();

        assert_eq!(column_def.data_type.name, date_type.name);
        assert_eq!(column_def.data_type.size, date_type.size);
        assert_eq!(column_def.name, name);
        assert_eq!(column_def.not_null, not_null);
        assert_eq!(column_def.primary_key, primary_key);
    }

    #[rstest]
    #[case("'22")]
    #[case("'22''")]
    #[case("3Id INT")]
    #[case("Id, INT")]
    #[case("Id INT PRIMARY")]
    #[case("Id INT NOT")]
    #[case("Id TEXT NULL")]
    fn test_column_def_failure(#[case] input_value: &str) {
        assert!(table::ColumnDefExprParser::new()
            .parse(input_value)
            .is_err());
    }
}
