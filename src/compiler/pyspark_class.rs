use serde::Serialize;
use tera::{Context, Tera};

use crate::model::{data_class::DataClass, table_expr::TableDef};

#[derive(Debug, Serialize)]
pub struct PySparkDataClassColumn {
    name: String,
    ref_name: String,
    not_null: bool,
    data_class: DataClass,
}

pub fn to_snake_case(name: &str) -> String {
    let mut s = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            // characters without capitalization are considered lowercase
            if i != 0 {
                s.push('_');
            }
            s.extend(c.to_lowercase());
        } else {
            s.push(c);
        }
    }
    s
}

/// Convert string like types to string since pyspark does not support string like types beside string
fn convert_string_like_type(_type: DataClass) -> DataClass {
    if _type.is_string_like() {
        return DataClass::String;
    }
    _type
}

pub fn compile(table_def: TableDef) -> String {
    let tera = match Tera::new("templates/gen_pyspark/**/*.py") {
        Ok(t) => t,
        Err(e) => {
            panic!("Parsing error(s): {}", e);
        }
    };

    let mut context = Context::new();

    let columns = table_def
        .columns
        .iter()
        .map(|x| PySparkDataClassColumn {
            name: x.name.clone(),
            ref_name: to_snake_case(x.name.as_str()),
            not_null: x.not_null,
            data_class: convert_string_like_type(x.data_type.class.clone()),
        })
        .collect::<Vec<PySparkDataClassColumn>>();

    context.insert("columns", &columns);
    context.insert(
        "table_name",
        &table_def.table_ref.to_string().replace('.', ""),
    );

    tera.render("data_class_sub.py", &context)
        .unwrap()
        .replace('\r', "")
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::compiler::test_strings::pyspark_class::PYSPARK_CLASS_EXPECTED_TEST_1;
    use crate::model::column_rule::{
        ColumnRule, ContainsValue, IsType, LikePattern, NonNull, NotEmpty, RegexPattern, Uniqueness,
    };
    use crate::model::rule_filter::filter::{ColumnRuleFilter, FilterCondition};
    use crate::model::rule_filter::operator::ComparisonOperator;
    use crate::model::table_expr::{ColumnDef, DataType, TableDef, TableRef};

    #[test]
    fn test_compile() {
        let table = TableDef {
            table_ref: TableRef::new("Test", Some("Schema"), None),
            columns: vec![
                ColumnDef {
                    name: "IdCol".to_string(),
                    data_type: DataType::new("VarChar", Some(3), None),
                    not_null: true,
                    primary_key: true,
                    rules: vec![
                        ColumnRuleFilter::new(
                            Some("Price > 1".to_string()),
                            vec![
                                ColumnRule::LikePattern(LikePattern {
                                    pattern: "%test%".to_string(),
                                    threshold: 0.5,
                                    ..Default::default()
                                }),
                                ColumnRule::ContainsValue(ContainsValue {
                                    value: "test".to_string(),
                                    threshold: 0.9,
                                    ..Default::default()
                                }),
                            ],
                        ),
                        ColumnRuleFilter::new(
                            None,
                            vec![
                                ColumnRule::NonNull(NonNull {
                                    name: "".to_string(),
                                    ..Default::default()
                                }),
                                ColumnRule::Uniqueness(Uniqueness {
                                    name: "".to_string(),
                                    ..Default::default()
                                }),
                                ColumnRule::IsType(IsType {
                                    name: "".to_string(),
                                    data_type: DataType::new("VarChar", Some(10), None),
                                    ..Default::default()
                                }),
                                ColumnRule::NotEmpty(NotEmpty {
                                    threshold: 0.9,
                                    ..Default::default()
                                }),
                                ColumnRule::Uniqueness(Uniqueness {
                                    ..Default::default()
                                }),
                            ],
                        ),
                    ],
                },
                ColumnDef {
                    name: "Price".to_string(),
                    data_type: DataType::new("VarChar", Some(10), None),
                    not_null: false,
                    primary_key: false,
                    rules: vec![
                        ColumnRuleFilter {
                            filter_condition: Some(FilterCondition::ValueCondition {
                                field: "Price".to_owned(),
                                operator: ComparisonOperator::GreaterThan,
                                value: "1".to_owned(),
                            }),
                            filter_string: Some("Price > 1".to_string()),
                            rules: vec![ColumnRule::IsType(IsType {
                                name: "".to_string(),
                                data_type: DataType::new("VarChar", Some(10), None),
                                ..Default::default()
                            })],
                        },
                        ColumnRuleFilter::new(
                            None,
                            vec![
                                ColumnRule::LikePattern(LikePattern {
                                    pattern: "%test%".to_string(),
                                    threshold: 0.5,
                                    ..LikePattern::default()
                                }),
                                ColumnRule::RegexPattern(RegexPattern {
                                    pattern: "[0-9]*test[0-9]*".to_string(),
                                    threshold: 0.75,
                                    ..Default::default()
                                }),
                                ColumnRule::NotEmpty(NotEmpty {
                                    threshold: 0.75,
                                    ..Default::default()
                                }),
                            ],
                        ),
                    ],
                },
                ColumnDef {
                    name: "Test".to_string(),
                    data_type: DataType::new("FLOAT", Some(3), None),
                    not_null: false,
                    primary_key: false,
                    rules: vec![],
                },
            ],
        };

        let compiled = compile(table);
        assert_eq!(PYSPARK_CLASS_EXPECTED_TEST_1, compiled);
    }
}
