#![cfg(test)]

use crate::model::data_class::DataClass;
use crate::model::table_expr::DataType;
use lalrpop_util::lalrpop_mod;
use rstest::rstest;

lalrpop_mod!(pub data_class, "/parser/data_class_parsing.rs");

#[rstest]
#[case("Tinyblob", DataClass::TinyBlob)]
#[case("Tinytext", DataClass::TinyText)]
fn test_no_size_type(#[case] input_str: &str, #[case] expected: DataClass) {
    let actual = data_class::NoSizeDataClassParser::new()
        .parse(input_str)
        .unwrap();
    assert_eq!(actual, expected);
    let actual = data_class::AllDataClassExprParser::new()
        .parse(input_str)
        .unwrap();
    assert_eq!(actual, expected);

    assert_eq!(
        data_class::OneSizeDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
    assert_eq!(
        data_class::TwoSizesDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
    assert_eq!(
        data_class::BothSizesDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
}

#[rstest]
#[case("cHar", DataClass::Char)]
#[case("VarChaR", DataClass::VarChar)]
#[case("BINARY", DataClass::Binary)]
#[case("int", DataClass::Int)]
#[case("teXT", DataClass::Text)]
fn test_one_size_type(#[case] input_str: &str, #[case] expected: DataClass) {
    let actual = data_class::OneSizeDataClassParser::new()
        .parse(input_str)
        .unwrap();
    assert_eq!(actual, expected);
    let actual = data_class::AllDataClassExprParser::new()
        .parse(input_str)
        .unwrap();
    assert_eq!(actual, expected);

    assert_eq!(
        data_class::NoSizeDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
    assert_eq!(
        data_class::TwoSizesDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
    assert_eq!(
        data_class::BothSizesDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
}

#[rstest]
#[case("double", DataClass::Double)]
#[case("DECIMAL", DataClass::Decimal)]
fn test_two_size_type(#[case] input_str: &str, #[case] expected: DataClass) {
    let actual = data_class::TwoSizesDataClassParser::new()
        .parse(input_str)
        .unwrap();
    assert_eq!(actual, expected);
    let actual = data_class::AllDataClassExprParser::new()
        .parse(input_str)
        .unwrap();
    assert_eq!(actual, expected);

    assert_eq!(
        data_class::NoSizeDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
    assert_eq!(
        data_class::OneSizeDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
    assert_eq!(
        data_class::BothSizesDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
}

#[rstest]
#[case("Float", DataClass::Float)]
fn test_both_size_type(#[case] input_str: &str, #[case] expected: DataClass) {
    let actual = data_class::BothSizesDataClassParser::new()
        .parse(input_str)
        .unwrap();
    assert_eq!(actual, expected);
    let actual = data_class::AllDataClassExprParser::new()
        .parse(input_str)
        .unwrap();
    assert_eq!(actual, expected);

    assert_eq!(
        data_class::NoSizeDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
    assert_eq!(
        data_class::OneSizeDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
    assert_eq!(
        data_class::TwoSizesDataClassParser::new()
            .parse(input_str)
            .is_err(),
        true
    );
}

#[rstest]
#[case("TinyBlob", DataType {class: DataClass::TinyBlob, size: None})]
#[case("Float(30)", DataType {class: DataClass::Float, size: Some([Some(30), None])})]
#[case("Float(30,5)", DataType {class: DataClass::Float, size: Some([Some(30), Some(5)])})]
#[case("VarChaR(30)", DataType {class: DataClass::VarChar, size: Some([Some(30), None])})]
#[case("double(30,5)", DataType {class: DataClass::Double, size: Some([Some(30), Some(5)])})]
fn test_data_type_expr(#[case] input_str: &str, #[case] expected: DataType) {
    let actual = data_class::DataTypeExprParser::new()
        .parse(input_str)
        .unwrap();
    assert_eq!(actual, expected);
}

#[rstest]
#[case("TinyBlob(30)")]
#[case("Float")]
#[case("double")]
#[case("VarChaR(30, 4)")]
#[case("double(30)")]
#[case("double(30,5")]
#[should_panic]
fn test_data_type_expr_failure(#[case] input_str: &str) {
    data_class::DataTypeExprParser::new()
        .parse(input_str)
        .unwrap();
}
