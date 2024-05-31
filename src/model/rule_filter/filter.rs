use lalrpop_util::lalrpop_mod;
use std::fmt::Display;

use crate::model::column_rule::ColumnRule;
use crate::model::rule_traits::{ColumnValidationError, ValidColumnRule};
use crate::model::table_expr::ColumnDef;
use crate::parser::error_utils::DDLxParseError;

use super::operator::ComparisonOperator;

lalrpop_mod!(pub rule, "/parser/rule_filter_expr.rs");

#[derive(Clone, Debug)]
pub enum FilterCondition {
    And(Vec<FilterCondition>),
    Or(Vec<FilterCondition>),
    Not(Box<FilterCondition>),
    FieldCondition {
        first_field: String,
        operator: ComparisonOperator,
        second_field: String,
    },
    ValueCondition {
        field: String,
        operator: ComparisonOperator,
        value: String,
    },
}

impl PartialEq for FilterCondition {
    fn eq(&self, other: &Self) -> bool {
        self.sort().to_string() == other.sort().to_string()
    }
}

impl Display for FilterCondition {
    #[allow(clippy::needless_return)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterCondition::And(conditions) => {
                let filter_conditions_string = conditions
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" AND ");
                return write!(f, "( {} )", filter_conditions_string);
            }
            FilterCondition::Or(conditions) => {
                let filter_conditions_string = conditions
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(" OR ");
                return write!(f, "( {} )", filter_conditions_string);
            }
            FilterCondition::Not(condition) => {
                return write!(f, "NOT ( {} )", condition);
            }
            FilterCondition::FieldCondition {
                first_field,
                operator,
                second_field,
            } => {
                let str = format!("( {} {} {} )", first_field, operator, second_field);
                return write!(f, "{}", str);
            }
            FilterCondition::ValueCondition {
                field,
                operator,
                value,
            } => {
                return write!(f, "( {} {} {} )", field, operator, value);
            }
        };
    }
}

impl FilterCondition {
    #[allow(clippy::needless_return)]
    pub fn reduce_nesting(&self) -> Self {
        match self {
            FilterCondition::And(conditions) => {
                let mut new_conditions: Vec<FilterCondition> = Vec::new();
                for condition in conditions {
                    let new_condition = condition.reduce_nesting();
                    match new_condition {
                        FilterCondition::And(nested_conditions) => {
                            let sub_conditions: Vec<FilterCondition> = nested_conditions
                                .iter()
                                .map(|c| c.reduce_nesting())
                                .collect();
                            new_conditions.extend(sub_conditions);
                        }
                        _ => {
                            new_conditions.push(new_condition);
                        }
                    }
                }

                return Self::And(new_conditions);
            }
            FilterCondition::Or(conditions) => {
                let mut new_conditions: Vec<FilterCondition> = Vec::new();
                for condition in conditions {
                    let new_condition = condition.reduce_nesting();
                    match new_condition {
                        FilterCondition::Or(nested_conditions) => {
                            let sub_conditions: Vec<FilterCondition> = nested_conditions
                                .iter()
                                .map(|c| c.reduce_nesting())
                                .collect();
                            new_conditions.extend(sub_conditions);
                        }
                        _ => {
                            new_conditions.push(new_condition);
                        }
                    }
                }

                return Self::Or(new_conditions);
            }
            FilterCondition::Not(condition) => {
                return Self::Not(Box::new(condition.reduce_nesting()));
            }
            _ => return self.to_owned(),
        }
    }

    pub fn from_str(filter_string: String) -> Result<FilterCondition, DDLxParseError> {
        let parsed = rule::RuleFilterExprParser::new().parse(&filter_string);

        if parsed.is_err() {
            return Err(DDLxParseError::InvalidFilterCondition(format!(
                "Failed to parse filter string: {}",
                filter_string
            )));
        }

        Ok(parsed.unwrap().reduce_nesting())
    }

    #[allow(clippy::needless_return)]
    pub fn sort(&self) -> Self {
        match self {
            FilterCondition::And(conditions) => {
                let sorted_conditions = Self::sort_conditions(conditions);
                return FilterCondition::And(sorted_conditions);
            }
            FilterCondition::Or(conditions) => {
                let sorted_conditions = Self::sort_conditions(conditions);
                return FilterCondition::Or(sorted_conditions);
            }
            FilterCondition::Not(condition) => {
                return Self::Not(Box::new(condition.sort()));
            }
            _ => self.to_owned(),
        }
    }

    fn sort_conditions(conditions: &[FilterCondition]) -> Vec<FilterCondition> {
        let mut sorted_conditions = conditions
            .iter()
            .map(|x| {
                let s = x.sort();
                #[allow(clippy::needless_return)]
                return (s.clone().to_string(), x.to_owned());
            })
            .collect::<Vec<(String, Self)>>();
        sorted_conditions.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let filter_conditions = sorted_conditions
            .iter()
            .map(|x| x.1.to_owned())
            .collect::<Vec<FilterCondition>>();
        filter_conditions
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnRuleFilter {
    pub filter_string: Option<String>,
    pub rules: Vec<ColumnRule>,
    pub filter_condition: Option<FilterCondition>,
}

impl Display for ColumnRuleFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if (self.filter_string.as_ref().is_some()
            && !self.filter_string.as_ref().unwrap().is_empty())
            && self.filter_condition.is_none()
        {
            panic!(
                "Calling to string on column filter {:?} before parsing it",
                self
            );
        }

        if self.filter_condition.is_none() {
            return write!(f, "");
        }

        write!(f, "{}", self.filter_condition.as_ref().unwrap())
    }
}

impl ColumnRuleFilter {
    pub fn empty_fr_rules(rules: Vec<ColumnRule>) -> ColumnRuleFilter {
        ColumnRuleFilter {
            filter_string: None,
            rules,
            filter_condition: None,
        }
    }

    pub fn from_rule(filter_string: Option<String>, rule: ColumnRule) -> ColumnRuleFilter {
        ColumnRuleFilter::new(filter_string, vec![rule])
    }

    pub fn new(filter_string: Option<String>, rules: Vec<ColumnRule>) -> ColumnRuleFilter {
        ColumnRuleFilter {
            filter_string,
            rules,
            filter_condition: None,
        }
    }

    pub fn parse(&self) -> Result<Self, DDLxParseError> {
        if self.filter_string.is_none() {
            return Ok(ColumnRuleFilter {
                filter_string: None,
                rules: self.rules.to_owned(),
                filter_condition: None,
            });
        }

        let filter_condition =
            FilterCondition::from_str(self.filter_string.as_ref().unwrap().clone())?;

        Ok(ColumnRuleFilter {
            filter_string: self.filter_string.clone(),
            rules: self.rules.to_owned(),
            filter_condition: Some(filter_condition),
        })
    }
}

impl ValidColumnRule for ColumnRuleFilter {
    fn validate_col_type(&self, column: &ColumnDef) -> Result<String, ColumnValidationError> {
        for rule in &self.rules {
            rule.validate_col_type(column)?;
        }

        Ok("valid".to_string())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::model::column_rule;

    use super::*;

    #[rstest]
    #[case(
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
                 ]),
                 FilterCondition::And(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ]),
            ]))),
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ]),
            )
        )
    )]
    #[case(
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
                 ]),
                 FilterCondition::Or(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ]),
            ]))),
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    FilterCondition::Or(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}])
                ]),
            )
        )
    )]
    #[case(
        FilterCondition::And(vec![
            FilterCondition::And(vec![
                FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                FilterCondition::And(vec![
                    FilterCondition::And(vec![
                        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    ]),
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ])
                ]),
        ]),
        FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
            ]),
        )
    ]
    #[case(
        FilterCondition::Or(vec![
            FilterCondition::Or(vec![
                FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                FilterCondition::Or(vec![
                    FilterCondition::Or(vec![
                        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    ]),
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ])
            ]),
        ]),
        FilterCondition::Or(vec![
            FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
            FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
            ]),
        )
    ]
    fn test_reduce_nesting(
        #[case] input_condition: FilterCondition,
        #[case] desired_condition: FilterCondition,
    ) {
        let actual_condition = input_condition.reduce_nesting();
        assert_eq!(actual_condition, desired_condition);
    }

    #[rstest]
    #[case(
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
                 ]),
                 FilterCondition::And(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ]),
            ]))),
        "NOT ( ( ( ( foo = 0  ) AND ( foo < bar ) ) AND ( ( foo = bar ) AND ( fizz = 0  ) ) ) )"
    )]
    #[case(
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ]),
            )
        ), "NOT ( ( ( foo = 0  ) AND ( foo < bar ) AND ( foo = bar ) AND ( fizz = 0  ) ) )"
    )]
    #[case(
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
                 ]),
                 FilterCondition::Or(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ]),
            ]))),
        "NOT ( ( ( ( foo = 0  ) AND ( foo < bar ) ) AND ( ( foo = bar ) OR ( fizz = 0  ) ) ) )"
    )]
    #[case(
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    FilterCondition::Or(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}])
                ]),
            )
        ), "NOT ( ( ( foo = 0  ) AND ( foo < bar ) AND ( ( foo = bar ) OR ( fizz = 0  ) ) ) )"
    )]
    #[case(
        FilterCondition::And(vec![
            FilterCondition::And(vec![
                FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                FilterCondition::And(vec![
                    FilterCondition::And(vec![
                        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    ]),
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ])
                ]),
        ]), "( ( ( foo = 0  ) AND ( ( ( foo = bar ) AND ( foo < bar ) ) AND ( fizz = 0  ) ) ) )")
    ]
    #[case(
        FilterCondition::Or(vec![
            FilterCondition::Or(vec![
                FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                FilterCondition::Or(vec![
                    FilterCondition::Or(vec![
                        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    ]),
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ])
            ]),
        ]),
        "( ( ( foo = 0  ) OR ( ( ( foo = bar ) OR ( foo < bar ) ) OR ( fizz = 0  ) ) ) )"
        )
    ]
    #[case(
        FilterCondition::Or(vec![
            FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
            FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
            ]),
        "( ( foo = 0  ) OR ( foo = bar ) OR ( foo < bar ) OR ( fizz = 0  ) )")
    ]
    fn test_filter_condition_to_string(
        #[case] input_condition: FilterCondition,
        #[case] desired_string: &str,
    ) {
        let actual_string = input_condition.to_string();
        assert_eq!(actual_string, desired_string);
    }

    #[rstest]
    #[case(
        FilterCondition::Or(vec![
            FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
            FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
            ]),
        FilterCondition::Or(vec![
            FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()},
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
            FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
            ])
        )
    ]
    #[case(
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
                 ]),
                 FilterCondition::Or(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ]),
            ]))),
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                FilterCondition::Or(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()},
                ]),
                FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    ]),
            ])))
    )]
    fn test_sort_filter_condition(
        #[case] input_condition: FilterCondition,
        #[case] desired_condition: FilterCondition,
    ) {
        let actual_condition = input_condition.sort();
        assert_eq!(actual_condition, desired_condition);
    }

    #[rstest]
    #[case(
        FilterCondition::Or(vec![
            FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
            FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
            ]),
        FilterCondition::Or(vec![
            FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()},
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
            FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
            ])
        )
    ]
    #[case(
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
                 ]),
                 FilterCondition::Or(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
                ]),
            ]))),
        FilterCondition::Not(Box::new(
            FilterCondition::And(vec![
                FilterCondition::Or(vec![
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                    FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()},
                ]),
                FilterCondition::And(vec![
                    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
                    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
                    ]),
            ])))
    )]
    fn test_filter_condition_eq(
        #[case] input_condition: FilterCondition,
        #[case] desired_condition: FilterCondition,
    ) {
        assert_eq!(input_condition, desired_condition);
    }

    #[test]
    #[should_panic]
    fn test_column_rule_filter_to_str_failure() {
        let column_rule = ColumnRuleFilter {
            filter_string: Some("Hello".to_owned()),
            rules: vec![],
            filter_condition: None,
        };

        column_rule.to_string();
    }

    #[rstest]
    #[case(ColumnRuleFilter {
        filter_string: Some("".to_owned()),
        rules: vec![],
        filter_condition: None,}
        , "")]
    #[case(ColumnRuleFilter {
            filter_string: Some("f".to_owned()),
            rules: vec![],
            filter_condition: Some(FilterCondition::Or(vec![
                FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
                FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()},
            ])),}
            , "( ( foo = bar ) OR ( fizz = 0  ) )")]
    fn test_column_rule_filter_to_str(
        #[case] column_rule: ColumnRuleFilter,
        #[case] expected: &str,
    ) {
        let actual = column_rule.to_string();
        assert_eq!(actual, expected);
    }
}
