#![cfg(test)]

use lalrpop_util::lalrpop_mod;
use rstest::rstest;

use crate::model::column_rule::{
    ColumnRule, ContainsValue, IsType, LikePattern, NonNull, NotEmpty, RegexPattern, Uniqueness,
};
use crate::model::data_class::DataClass;
use crate::model::rule_ext_config::RuleExtConfig;
use crate::model::rule_filter::filter::ColumnRuleFilter;
use crate::model::table_expr::DataType;
use crate::model::table_expr::{ColumnDef, TableRef};

lalrpop_mod!(pub table, "/parser/create_table.rs");

#[rstest]
#[case("CREATE TABLE IF NOT EXISTS Inventory {Id INT(10),Title VARCHAR(3),  };",
TableRef::new("Inventory", None, None),
vec![
    ColumnDef {name: String::from("Id"), data_type: DataType::new("INT", Some(10), None), rules:
    vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::Int, size: Some([Some(10), None]) }, ..Default::default()})])],
    ..Default::default()},
    ColumnDef {name: String::from("Title"), data_type: DataType::new("VARCHAR", Some(3), None), rules:
    vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::VarChar, size: Some([Some(3), None]) }, ..Default::default()})])],
    ..Default::default()},
])]
#[case("CREATE TABLE IF NOT EXISTS\n Test {Id FLOAT(10)};", "Test",
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(10), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(10), None]) }, ..Default::default()})])],
..Default::default()},
])]
#[case(" CREATE TABLE IF NOT EXISTS\n Test \n{\nId FLOAT(2)\n,}\n;\n", "Test",
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(2), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(2), None]) }, ..Default::default()})])], ..Default::default()},
])]
#[case(" create table if not exists\n Test \n{\nId FLOAT(3)\n,}\n;\n", "Test",
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(3), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(3), None]) }, ..Default::default()})])], ..Default::default()},
])]
#[case(" create table if not exists\n Schema.Test \n{\nId FLOAT(1)\n,}\n;\n",
TableRef::new("Test", Some("Schema"), None),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(1), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(1), None]) }, ..Default::default()})])], ..Default::default()},
])]
#[case(" create table if not exists\n Schema.Test \'jlk asdf19(**\' \n{\nId FLOAT(3)\n,}\n;\n",
TableRef::new("Test", Some("Schema"), Some("jlk asdf19(**")),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(3), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(3), None]) }, ..Default::default()})])],..Default::default()},
])]
#[case(" create table if not exists\n Schema.Test \"jlk asdf19(**\" \n{\nId FLOAT(9)\n,}\n;\n",
TableRef::new("Test", Some("Schema"), Some("jlk asdf19(**")),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(9), None), rules: 
vec![ColumnRuleFilter::new(None, vec![
ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(9), None]) }, ..Default::default()})
])], ..Default::default()},
])]
#[case(" create table if not exists\n Test \n{\nId FLOAT(2)\n,}\n;\n",
TableRef::new("Test", None, None),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(2), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(2), None]) }, ..Default::default()})])], ..Default::default()},
])]
#[case(" CREATE TABLE IF NOT EXISTS\n Test33 \n{\nId FLOAT(2),\nPrice FLOAT(3),\nNotes TEXT(10)\n}\n;\n",
TableRef::new("Test33", None, None),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(2), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(2), None]) }, ..Default::default()})])],..Default::default()},
ColumnDef {name: String::from("Price"), data_type: DataType::new("FLOAT", Some(3), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(3), None]) }, ..Default::default()})])], ..Default::default()},
ColumnDef {name: String::from("Notes"), data_type: DataType::new("TEXT", Some(10), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Text, size: Some([Some(10), None]) }, ..Default::default()})])], ..Default::default()},
])]
#[case(" create table\n Test33 \n{\nId FLOAT(1) PRIMARY KEY,\nPrice FLOAT(2),\nNotes TEXT(10) not null\n}\n;\n",
TableRef::new("Test33", None, None),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(1), None), primary_key: true, not_null: true, rules: 
vec![ColumnRuleFilter::new(None, vec![
ColumnRule::NonNull(NonNull::new(None, None, None)), ColumnRule::Uniqueness(Uniqueness::new(None, None)),
ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(1), None]) }, ..Default::default()})])], ..Default::default()},

ColumnDef {name: String::from("Price"), data_type: DataType::new("FLOAT", Some(2), None), rules:
vec![ColumnRuleFilter::new(None, vec![ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(2), None]) }, ..Default::default()})])], ..Default::default()},

ColumnDef {name: String::from("Notes"), data_type: DataType::new("TEXT", Some(10), None), not_null: true, rules: 
vec![ColumnRuleFilter::new(None, vec![
ColumnRule::NonNull(NonNull::new(None, None, None)),
ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Text, size: Some([Some(10), None]) }, ..Default::default()})])],..Default::default()},
])]
#[case(" create table if not exists\n Test \n{\nId FLOAT(100)\n { -LIKE \"%test%\" },}\n;\n",
TableRef::new("Test", None, None),
vec![
ColumnDef {name: String::from("Id"), data_type: DataType::new("FLOAT", Some(100), None), rules: 
vec![ColumnRuleFilter::new(None, vec![
ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
class: DataClass::Float, size: Some([Some(100), None]) }, ..Default::default()})]),
ColumnRuleFilter::new(None, vec![
ColumnRule::LikePattern(LikePattern  {name: String::new(), rule_ext_config: RuleExtConfig::new_empty(),
pattern: "%test%".to_owned(), ..Default::default()}),
])], ..Default::default()},
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
        assert_eq!(column, desired_col);
    }
}

#[rstest]
#[case("CREATE TABLE IF NOT EXISTS Inventory {Id INT,Title VARCHAR,  }")]
#[case("CREATE TABLE IF NOT EXISTS Inventory {Id INT Title VARCHAR,  };")]
#[case("CREATE TABLE IF NOT EXISTS {Id INT,Title VARCHAR,  };")]
#[case("CREATE TABLE IF NOT EXISTS Inventory {-Id INT,Title VARCHAR,  };")]
#[case(" create table if not exists\n Schema.Test \"jlk \'asdf19(**\" \n{\nId FLOAT\n,}\n;\n")]
#[should_panic]
fn test_create_table_failure(#[case] input_value: &str) {
    table::CreateTableExprParser::new()
        .parse(input_value)
        .unwrap();
}

#[rstest]
#[case("Id INT(10)", "Id", DataType::new("INT", Some(10), None), false, false)]
#[case("Id  INT(5) ", "Id", DataType::new("INT", Some(5), None), false, false)]
#[case(" Id INT(3)", "Id", DataType::new("INT", Some(3), None), false, false)]
#[case(
    " Id33 INT(1)",
    "Id33",
    DataType::new("INT", Some(1), None),
    false,
    false
)]
#[case(
    " Id33 INT(1) PRIMARY KEY",
    "Id33",
    DataType::new("INT", Some(1), None),
    true,
    true
)]
#[case(
    " Id33 TinyText primaRY KeY",
    "Id33",
    DataType::new("TinyText", None, None),
    true,
    true
)]
#[case(
    " Id33 TinyText NOT NULL",
    "Id33",
    DataType::new("TinyText", None, None),
    true,
    false
)]
#[case(
    "_Id-3_3 TinyText NOT NULL",
    "_Id-3_3",
    DataType::new("TinyText", None, None),
    true,
    false
)]
#[case(
    " Id33 DOUBLE(1,2) not null",
    "Id33",
    DataType::new("DOUBLE", Some(1), Some(2)),
    true,
    false
)]
#[case(
    " Id33 TinyText not NuLL",
    "Id33",
    DataType::new("TinyText", None, None),
    true,
    false
)]
#[case(
    " Id33 DOUBLE(30,2) not NuLL",
    "Id33",
    DataType::new("DOUBLE", Some(30), Some(2)),
    true,
    false
)]
#[case(
    " Id33 DOUBLE(30,5) not NuLL",
    "Id33",
    DataType {class: DataClass::Double, size: Some([Some(30), Some(5)])},
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
#[case("ISBN VARCHAR(20) { -LIKE \"%test%\" ")]
#[case("ISBN VARCHAR(20) { LIKE \"%test%\" }")]
fn test_column_def_failure(#[case] input_value: &str) {
    assert!(table::ColumnDefExprParser::new()
        .parse(input_value)
        .is_err());
}

#[rstest]
#[case("ISBN VARCHAR(20) { -REGEX \"^(?=(?:\\D*\\d){10}(?:(?:\\D*\\d){3})?$)[\\d-]+$\" }", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::new("VARCHAR", Some(20), None),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRuleFilter::new(None, vec![
    ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::VarChar, size: Some([Some(20), None]) }, ..Default::default()})]),
    ColumnRuleFilter::new(None, vec![
    ColumnRule::RegexPattern(RegexPattern {name: String::new(), rule_ext_config: RuleExtConfig::new_empty(),
    pattern: "^(?=(?:\\D*\\d){10}(?:(?:\\D*\\d){3})?$)[\\d-]+$".to_owned(), ..Default::default()})])]
})]
#[case("ISBN VARCHAR(20) { -LIKE \"%test%\" }", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::new("VARCHAR", Some(20), None),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRuleFilter::new(None, vec![
    ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::VarChar, size: Some([Some(20), None]) }, ..Default::default()})]),
    ColumnRuleFilter::new(None, vec![
    ColumnRule::LikePattern(LikePattern  {name: String::new(), rule_ext_config: RuleExtConfig::new_empty(),
    pattern: "%test%".to_owned(), ..Default::default()})])]
})]
#[case("ISBN VARCHAR(20) { -CONTAINS \"test\" }", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::new("VARCHAR", Some(20), None),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRuleFilter::new(None, vec![
    ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::VarChar, size: Some([Some(20), None]) }, ..Default::default()})]),
    ColumnRuleFilter::new(None, vec![
    ColumnRule::ContainsValue(ContainsValue  {name: String::new(), rule_ext_config: RuleExtConfig::new_empty(),
    value: "test".to_owned(), ..Default::default()})])]
})]
#[case("ISBN VARCHAR(20) { -CONTAINS \"test\" 0.01 }", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::new("VARCHAR", Some(20), None),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRuleFilter::new(None, vec![
    ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::VarChar, size: Some([Some(20), None]) }, ..Default::default()})]),
    ColumnRuleFilter::new(None, vec![
    ColumnRule::ContainsValue(ContainsValue  {name: String::new(), rule_ext_config: RuleExtConfig::new_empty(),
    threshold: 0.01, value: "test".to_owned(), ..Default::default()})])]
})]
#[case("ISBN VARCHAR(20) { -CONTAINS \"test\" 1. }", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::new("VARCHAR", Some(20), None),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRuleFilter::new(None, vec![
    ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::VarChar, size: Some([Some(20), None]) }, ..Default::default()})]),
    ColumnRuleFilter::new(None, vec![
    ColumnRule::ContainsValue(ContainsValue  {name: String::new(), rule_ext_config: RuleExtConfig::new_empty(),
    value: "test".to_owned(), ..Default::default()})])]
})]
#[case("ISBN VARCHAR(20) PRIMARY KEY ", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::new("VARCHAR", Some(20), None),
    not_null: true,
    primary_key: true,
    rules: vec![ColumnRuleFilter::new(None, vec![
    ColumnRule::NonNull(NonNull::new(None, None, None)),
    ColumnRule::Uniqueness(Uniqueness::new(None, None)),
    ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::VarChar, size: Some([Some(20), None]) }, ..Default::default()}),])]
})]
#[case("ISBN VARCHAR(20) { -unique} ", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::new("VARCHAR", Some(20), None),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRuleFilter::new(None, vec![
    ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::VarChar, size: Some([Some(20), None]) }, ..Default::default()})]),
    ColumnRuleFilter::new(None, vec![
    ColumnRule::Uniqueness(Uniqueness::new(None, None)),])]
})]
#[case("ISBN VARCHAR(20) { -not_empty} ", ColumnDef {
    name: String::from("ISBN"),
    data_type: DataType::new("VARCHAR", Some(20), None),
    not_null: false,
    primary_key: false,
    rules: vec![ColumnRuleFilter::new(None, vec![
    ColumnRule::IsType(IsType {  name: "".to_owned(), data_type: DataType {
    class: DataClass::VarChar, size: Some([Some(20), None]) }, ..Default::default()})]),
    ColumnRuleFilter::new(None, vec![
    ColumnRule::NotEmpty(NotEmpty::new(None, None, None)),])]
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
