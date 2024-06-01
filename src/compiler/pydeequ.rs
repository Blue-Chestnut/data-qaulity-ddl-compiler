use serde::Serialize;
use tera::{Context, Tera};

use crate::compiler::pydeequ::pydeequ_rule::compile_column_rule;
use crate::model::rule_filter::filter::ColumnRuleFilter;
use crate::model::table_expr::{ColumnDef, TableDef};

pub mod pydeequ_rule {
    use crate::model::column_rule::{
        ColumnRule, ContainsValue, IsType, LikePattern, NotEmpty, RegexPattern,
    };

    pub trait Compiling {
        fn compile(&self) -> String;
    }

    pub struct HasDataType {
        rule: IsType,
        column_name: String,
    }

    impl Compiling for HasDataType {
        fn compile(&self) -> String {
            if self.rule.data_type.class.is_string_like() {
                return format!(
                    ".hasDataType(\"{}\", ConstrainableDataTypes.String, lambda x: x >= 1)",
                    &self.column_name
                );
            } else if self.rule.data_type.class.is_fraction_like() {
                return format!(
                    ".hasDataType(\"{}\", ConstrainableDataTypes.Fractional, lambda x: x >= 1)",
                    &self.column_name
                );
            } else if self.rule.data_type.class.is_numeric_like() {
                return format!(
                    ".hasDataType(\"{}\", ConstrainableDataTypes.Numeric, lambda x: x >= 1)",
                    &self.column_name
                );
            } else if self.rule.data_type.class.is_boolean_like() {
                return format!(
                    ".hasDataType(\"{}\", ConstrainableDataTypes.Boolean, lambda x: x >= 1)",
                    &self.column_name
                );
            }
            panic!(
                "Cannot compile HasDataType for column {} with type {:?}",
                self.column_name, self.rule.data_type
            )
        }
    }

    pub struct HasPattern {
        rule: RegexPattern, // https://pydeequ.readthedocs.io/en/latest/pydeequ.html#pydeequ.checks.Check.hasPattern
        column_name: String,
        table_name: String,
    }

    impl Compiling for HasPattern {
        fn compile(&self) -> String {
            let constraint_name = format!(
                "check_has_pattern_{}_{}",
                &self.table_name, &self.column_name
            );
            format!(
                ".hasPattern(\"{}\", r\"{}\", lambda x: x >= {}, \"{}\")",
                &self.column_name, &self.rule.pattern, self.rule.threshold, constraint_name
            )
        }
    }

    pub struct Completeness {
        column_name: String,
        table_name: String,
    }

    impl Compiling for Completeness {
        fn compile(&self) -> String {
            let constraint_name = format!(
                "check_completeness_{}_{}",
                &self.table_name, &self.column_name
            );
            format!(
                ".isComplete(\"{}\", \"{}\")",
                &self.column_name, constraint_name
            )
        }
    }

    pub struct Uniqueness {
        column_name: String,
        table_name: String,
    }

    impl Compiling for Uniqueness {
        fn compile(&self) -> String {
            let constraint_name = format!(
                "check_uniqueness_{}_{}",
                &self.table_name, &self.column_name
            );
            format!(
                ".isUnique(\"{}\", \"{}\")",
                &self.column_name, constraint_name
            )
        }
    }

    pub struct SatisfiesLike {
        rule: LikePattern, //pydeequ https://pydeequ.readthedocs.io/en/latest/pydeequ.html#pydeequ.checks.Check.satisfies
        column_name: String,
        table_name: String,
    }

    impl Compiling for SatisfiesLike {
        fn compile(&self) -> String {
            let constraint_name = format!(
                "check_like_pattern_{}_{}",
                &self.table_name, &self.column_name
            );
            format!(
                ".satisfies(\"{} LIKE '{}'\", \"{}\", lambda x: x >= {})",
                &self.column_name, &self.rule.pattern, constraint_name, self.rule.threshold
            )
        }
    }

    pub struct ContainsString {
        rule: ContainsValue, // https://pydeequ.readthedocs.io/en/latest/pydeequ.html#pydeequ.checks.Check.hasPattern
        column_name: String,
        table_name: String,
    }

    impl Compiling for ContainsString {
        fn compile(&self) -> String {
            let constraint_name = format!(
                "check_contains_value_{}_{}",
                &self.table_name, &self.column_name
            );
            format!(
                ".hasPattern(\"{}\", r\"{}\", lambda x: x >= {}, \"{}\")",
                &self.column_name, &self.rule.value, self.rule.threshold, constraint_name
            )
        }
    }

    pub struct PydeequNotEmpty {
        rule: NotEmpty, // https://pydeequ.readthedocs.io/en/latest/pydeequ.html#pydeequ.checks.Check.hasPattern
        column_name: String,
        table_name: String,
    }

    impl Compiling for PydeequNotEmpty {
        fn compile(&self) -> String {
            let constraint_name =
                format!("check_not_empty_{}_{}", &self.table_name, &self.column_name);
            format!(
                ".satisfies(\"length({}) > 0\", \"{}\", lambda x: x >= {})",
                &self.column_name, constraint_name, self.rule.threshold
            )
        }
    }

    pub fn compile_column_rule(
        column_rule: ColumnRule,
        table_name: String,
        column_name: String,
    ) -> String {
        match column_rule {
            ColumnRule::RegexPattern(rule) => HasPattern {
                rule,
                column_name,
                table_name,
            }
            .compile(),
            ColumnRule::LikePattern(rule) => SatisfiesLike {
                rule,
                column_name,
                table_name,
            }
            .compile(),
            ColumnRule::ContainsValue(rule) => ContainsString {
                rule,
                column_name,
                table_name,
            }
            .compile(),
            ColumnRule::IsType(rule) => HasDataType { rule, column_name }.compile(),
            ColumnRule::Uniqueness(_) => Uniqueness {
                column_name: column_name.clone(),
                table_name: table_name.clone(),
            }
            .compile(),
            ColumnRule::NonNull(_) => Completeness {
                column_name: column_name.clone(),
                table_name: table_name.clone(),
            }
            .compile(),
            ColumnRule::NotEmpty(rule) => PydeequNotEmpty {
                column_name: column_name.clone(),
                table_name: table_name.clone(),
                rule,
            }
            .compile(), // _ => unimplemented!("Pydeequ has no implementation of rule: {:?}", column_rule),
        }
    }

    #[cfg(test)]
    pub mod test {
        use crate::compiler::pydeequ::pydeequ_rule::compile_column_rule;
        use crate::model::column_rule::{
            ColumnRule, ContainsValue, IsType, LikePattern, NonNull, NotEmpty, RegexPattern,
            Uniqueness,
        };
        use crate::model::table_expr::DataType;
        use rstest::rstest;

        #[rstest]
        #[case(ColumnRule::RegexPattern(RegexPattern {name: "".to_owned(), pattern: "^(?:\\D*\\d){10}$".to_owned(), threshold: 0.5, ..Default::default()}), "Test", "Id", ".hasPattern(\"Id\", r\"^(?:\\D*\\d){10}$\", lambda x: x >= 0.5, \"check_has_pattern_Test_Id\")")]
        #[case(ColumnRule::LikePattern(LikePattern {name: "".to_owned(), pattern: "%test%".to_owned(), ..Default::default()}), "Test", "Price", ".satisfies(\"Price LIKE '%test%'\", \"check_like_pattern_Test_Price\", lambda x: x >= 1)")]
        #[case(ColumnRule::ContainsValue(ContainsValue {name: "".to_owned(), value: "test".to_owned(), ..Default::default()}), "Test", "Id", ".hasPattern(\"Id\", r\"test\", lambda x: x >= 1, \"check_contains_value_Test_Id\")")]
        #[case(ColumnRule::Uniqueness(Uniqueness {name: "".to_owned(), ..Default::default()}), "Test", "Id", ".isUnique(\"Id\", \"check_uniqueness_Test_Id\")")]
        #[case(ColumnRule::NonNull(NonNull {name: "".to_owned(), ..Default::default()}), "Table", "Column", ".isComplete(\"Column\", \"check_completeness_Table_Column\")")]
        #[case(ColumnRule::IsType(IsType {name: "".to_owned(), data_type: DataType::new("INT", Some(4), None), ..Default::default()}), "Test", "Quantity", ".hasDataType(\"Quantity\", ConstrainableDataTypes.Numeric, lambda x: x >= 1)")]
        #[case(ColumnRule::IsType(IsType {name: "".to_owned(), data_type: DataType::new("VarChar", Some(4), None), ..Default::default()}), "Test", "Description", ".hasDataType(\"Description\", ConstrainableDataTypes.String, lambda x: x >= 1)")]
        #[case(ColumnRule::IsType(IsType {name: "".to_owned(), data_type: DataType::new("Float", Some(4), None), ..Default::default()}), "Test", "Price", ".hasDataType(\"Price\", ConstrainableDataTypes.Fractional, lambda x: x >= 1)")]
        #[case(ColumnRule::IsType(IsType {name: "".to_owned(), data_type: DataType::new("Bool", None, None), ..Default::default()}), "Test", "Available", ".hasDataType(\"Available\", ConstrainableDataTypes.Boolean, lambda x: x >= 1)")]
        #[case(ColumnRule::NotEmpty(NotEmpty {name: "".to_owned(), ..Default::default()}), "Test", "Value", ".satisfies(\"length(Value) > 0\", \"check_not_empty_Test_Value\", lambda x: x >= 1)")]
        pub fn test_compile_column_rule(
            #[case] column_rule: ColumnRule,
            #[case] table_name: String,
            #[case] column_name: String,
            #[case] expected: String,
        ) {
            let result = compile_column_rule(column_rule, table_name, column_name);
            assert_eq!(result, expected);
        }
    }
}

#[derive(Serialize)]
pub struct ColumnLevelFilter {
    pub has_filter: bool,
    pub checks: Vec<String>,
    pub filter: String,
    pub description: String,
}

impl ColumnLevelFilter {
    pub fn new(filter_rules: ColumnRuleFilter, column: &ColumnDef, table: &TableDef) -> Self {
        let mut checks = vec![String::new(); filter_rules.rules.len()];

        for (i, rule) in filter_rules.rules.iter().enumerate() {
            let rule_copy = rule.clone();

            let table_name = table.table_ref.to_string();
            let column_name = column.name.clone();
            checks[i] = compile_column_rule(rule_copy, table_name, column_name);
        }

        let filter = filter_rules.filter_string.unwrap_or_default();

        Self {
            has_filter: !filter.is_empty(),
            checks,
            filter: filter.clone(),
            description: format!(
                "Autogenerated check for column level rules for table {} and column {} with filter {}",
                table.table_ref,
                &column.name.as_str(),
                filter
            ),
        }
    }
}

#[derive(Serialize)]
pub struct ColumnLevelCheck {
    pub column_name: String,
    pub description: String,
    pub ext_column_name: String,
    pub filter_checks: Vec<ColumnLevelFilter>,
}

impl ColumnLevelCheck {
    pub fn new(column: ColumnDef, table: &TableDef) -> Self {
        let mut filter_checks: Vec<ColumnLevelFilter> = vec![];

        for rule_filter in column.rules.iter() {
            filter_checks.push(ColumnLevelFilter::new(rule_filter.clone(), &column, table));
        }

        Self {
            description: format!(
                "Autogenerated check for column level rules for table {} and column {}",
                table.table_ref,
                &column.name.as_str()
            ),
            column_name: column.name.to_lowercase(),
            filter_checks,
            ext_column_name: format!("{}.{}", table.table_ref, column.name.as_str()),
        }
    }
}

pub fn compile_column_level_checks(columns: Vec<ColumnDef>, table: &TableDef) -> String {
    let tera = match Tera::new("templates/pydeequ/**/*.py") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let mut context = Context::new();

    let column_level_checks: Vec<ColumnLevelCheck> = columns
        .iter()
        .filter(|column| !column.rules.is_empty())
        .map(|column| ColumnLevelCheck::new(column.clone(), table))
        .collect();

    context.insert("column_level_checks", &column_level_checks);

    tera.render("column_level_check.py", &context)
        .unwrap()
        .replace('\r', "")
}

pub fn compile(table: TableDef) -> String {
    compile_column_level_checks(table.columns.clone(), &table)
}

#[cfg(test)]
mod tests {

    use crate::compiler::test_strings::pydeequ::PYTHON_PYDEEQU_RESULT_1;
    use crate::model::column_rule::{
        ColumnRule, ContainsValue, IsType, LikePattern, NonNull, NotEmpty, RegexPattern, Uniqueness,
    };
    use crate::model::rule_filter::filter::{ColumnRuleFilter, FilterCondition};
    use crate::model::rule_filter::operator::ComparisonOperator;
    use crate::model::table_expr::{ColumnDef, DataType, TableDef, TableRef};

    #[test]
    pub fn compile_test() {
        let table = TableDef {
            table_ref: TableRef::new("Test", None, None),
            columns: vec![
                ColumnDef {
                    name: "Id".to_string(),
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

        let compiled = crate::compiler::pydeequ::compile(table);
        assert_eq!(PYTHON_PYDEEQU_RESULT_1, compiled);
    }
}
