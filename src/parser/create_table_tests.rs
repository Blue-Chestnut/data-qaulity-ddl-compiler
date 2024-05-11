use lalrpop_util::lalrpop_mod;
use rstest::rstest;

use crate::model::column_rule::ColumnRule;
use crate::model::rule_ext_config::RuleExtConfig;
use crate::model::table_expr::DataType;
use crate::model::table_expr::{ColumnDef, TableRef};

lalrpop_mod!(pub table, "/parser/create_table.rs");

#[rstest]
#[case("CREATE TABLE IF NOT EXISTS Inventory {Id INT,Title VARCHAR,  };",
TableRef::from_str("Inventory", None, None),
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
#[case(" create table if not exists\n Schema.Test \n{\nId FLOAT\n,}\n;\n",
TableRef::from_str("Test", Some("Schema"), None),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
])]
#[case(" create table if not exists\n Schema.Test \'jlk asdf19(**\' \n{\nId FLOAT\n,}\n;\n",
TableRef::from_str("Test", Some("Schema"), Some("jlk asdf19(**")),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
])]
#[case(" create table if not exists\n Schema.Test \"jlk asdf19(**\" \n{\nId FLOAT\n,}\n;\n",
TableRef::from_str("Test", Some("Schema"), Some("jlk asdf19(**")),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
])]
#[case(" create table if not exists\n Test \n{\nId FLOAT\n,}\n;\n",
TableRef::from_str("Test", None, None),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
])]
#[case(" CREATE TABLE IF NOT EXISTS\n Test33 \n{\nId FLOAT,\nPrice FLOAT,\nNotes TEXT\n}\n;\n",
TableRef::from_str("Test33", None, None),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
ColumnDef {name: String::from("Price"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
ColumnDef {name: String::from("Notes"), data_type: DataType::f_name("TEXT"), ..Default::default()},
])]
#[case(" create table\n Test33 \n{\nId FLOAT PRIMARY KEY,\nPrice FLOAT,\nNotes TEXT not null\n}\n;\n",
TableRef::from_str("Test33", None, None),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::f_name("FLOAT"), primary_key: true, not_null: true, ..Default::default()},
ColumnDef {name: String::from("Price"), data_type: DataType::f_name("FLOAT"), ..Default::default()},
ColumnDef {name: String::from("Notes"), data_type: DataType::f_name("TEXT"), not_null: true,..Default::default()},
])]
fn test_create_table_success(
    #[case] input_value: &str,
    #[case] table_ref: TableRef,
    #[case] cols: Vec<ColumnDef>,
) {
    let parsed_result = table::CreateTableExprParser::new().parse(input_value);
    let parsed_result_ref = parsed_result.as_ref();

    assert!(parsed_result_ref.is_ok(), "{:?}", parsed_result_ref.err());

    let column_def = parsed_result_ref.unwrap().as_ref();

    assert_eq!(column_def.table_ref.schema_name, table_ref.schema_name);
    assert_eq!(column_def.table_ref.table_name, table_ref.table_name);
    assert_eq!(column_def.table_ref.alias, table_ref.alias);
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
#[case("CREATE TABLE IF NOT EXISTS Inventory {-Id INT,Title VARCHAR,  };")]
#[case(" create table if not exists\n Schema.Test \"jlk \'asdf19(**\" \n{\nId FLOAT\n,}\n;\n")]
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
#[case(
    "_Id-3_3 TEXT NOT NULL",
    "_Id-3_3",
    DataType::f_name("TEXT"),
    true,
    false
)]
#[case(
    " Id33 DOUBLE not null",
    "Id33",
    DataType::f_name("DOUBLE"),
    true,
    false
)]
#[case(
    " Id33 DOUBLE not NuLL",
    "Id33",
    DataType::f_name("DOUBLE"),
    true,
    false
)]
#[case(
    " Id33 DOUBLE(30) not NuLL",
    "Id33",
    DataType::f_name_1_size("DOUBLE", 30),
    true,
    false
)]
#[case(
    " Id33 DOUBLE(30,5) not NuLL",
    "Id33",
    DataType {name: String::from("DOUBLE"), size: Some([Some(30), Some(5)])},
    true,
    false
)]
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

    let column_def = parsed_result_ref.unwrap();

    assert_eq!(column_def.data_type, date_type);
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
#[case("ISBN VACHAR(20) { -LIKE \"%test%\" ")]
#[case("ISBN VACHAR(20) { LIKE \"%test%\" }")]
fn test_column_def_failure(#[case] input_value: &str) {
    assert!(table::ColumnDefExprParser::new()
        .parse(input_value)
        .is_err());
}

#[rstest]
#[case("ISBN VACHAR(20) { -REGEX \"^(?=(?:\\D*\\d){10}(?:(?:\\D*\\d){3})?$)[\\d-]+$\" }", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::f_name_1_size("VACHAR", 20),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRule::RegexPattern {name: String::new(), rule_ext_config: RuleExtConfig::new_empty(), pattern: "^(?=(?:\\D*\\d){10}(?:(?:\\D*\\d){3})?$)[\\d-]+$".to_owned()}]
})]
#[case("ISBN VACHAR(20) { -LIKE \"%test%\" }", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::f_name_1_size("VACHAR", 20),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRule::LikePattern {name: String::new(), rule_ext_config: RuleExtConfig::new_empty(), pattern: "%test%".to_owned()}]
})]
#[case("ISBN VACHAR(20) { -CONTAINS \"test\" }", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::f_name_1_size("VACHAR", 20),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRule::ContainsValue {name: String::new(), rule_ext_config: RuleExtConfig::new_empty(), value: "test".to_owned()}]
})]
fn test_column_with_rule_expr_success(
    #[case] input_value: &str,
    #[case] desired_column: ColumnDef,
) {
    let parsed_result = table::ColumnWithRulesExprParser::new().parse(input_value);
    let parsed_result_ref = parsed_result.as_ref();

    assert!(parsed_result_ref.is_ok(), "{:?}", parsed_result_ref.err());

    let column_def = parsed_result_ref.unwrap();
    assert_eq!(column_def.rules, desired_column.rules);
    assert_eq!(column_def.name, desired_column.name);
    assert_eq!(column_def.data_type, desired_column.data_type);
    assert_eq!(column_def.not_null, desired_column.not_null);
    assert_eq!(column_def.primary_key, desired_column.primary_key);
}
