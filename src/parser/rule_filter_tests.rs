#![cfg(test)]

use lalrpop_util::lalrpop_mod;
use rstest::rstest;

use crate::model::rule_filter::{filter::FilterCondition, operator::ComparisonOperator};

lalrpop_mod!(pub rule, "/parser/rule_filter_expr.rs");

#[rstest]
#[case(">", ComparisonOperator::GreaterThan)]
#[case("<", ComparisonOperator::LessThan)]
#[case("=", ComparisonOperator::Equal)]
#[case("!=", ComparisonOperator::NotEqual)]
#[case(">=", ComparisonOperator::GreaterThanOrEqual)]
#[case("<=", ComparisonOperator::LessThanOrEqual)]
fn test_comparison_operator(#[case] input: &str, #[case] expected: ComparisonOperator) {
    let parsed = rule::ComparisonOperatorExprParser::new().parse(input);
    assert_eq!(expected, parsed.unwrap());
}

#[rstest]
#[case("foo > bar", FilterCondition::FieldCondition { first_field: "foo".to_owned(), 
operator: ComparisonOperator::GreaterThan, second_field: "bar".to_owned() })]
#[case("foo > 1.", FilterCondition::ValueCondition { field: "foo".to_owned(), 
operator: ComparisonOperator::GreaterThan, value: "1.".to_owned() })]
#[case("foo != 0.04", FilterCondition::ValueCondition { field: "foo".to_owned(), 
operator: ComparisonOperator::NotEqual, value: "0.04".to_owned() })]
#[case("foo <= -0.04", FilterCondition::ValueCondition { field: "foo".to_owned(), 
operator: ComparisonOperator::LessThanOrEqual, value: "-0.04".to_owned() })]
#[case("foo = 0.", FilterCondition::ValueCondition { field: "foo".to_owned(), 
operator: ComparisonOperator::Equal, value: "0.".to_owned() })]
#[case("(foo = 0 && foo < bar)", FilterCondition::And(vec![
    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
]))]
#[case("foo = 0 && ( foo < bar || foo = bar ) && fizz = 0", FilterCondition::And(vec![
    FilterCondition::And(vec![
        FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
        FilterCondition::Or(vec![
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
            FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() }
        ]),
     ]),
     FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0".to_owned()}
]))]
#[case("foo = 0 && foo < bar", FilterCondition::And(vec![
    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
]))]
#[case("foo = 0 || foo < bar", FilterCondition::Or(vec![
    FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
]))]
#[case("foo = 0 &&  foo < bar || foo = bar", FilterCondition::Or(vec![
    FilterCondition::And(vec![
        FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() },
    ]),
    FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() }
]))]
#[case("foo = 0 &&  foo < bar || foo = bar && fizz = 0", FilterCondition::Or(vec![
    FilterCondition::And(vec![
        FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
     ]),
     FilterCondition::And(vec![
        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
        FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0".to_owned()}
    ]),
]))]
#[case("!(foo = 0 && foo < bar || foo = bar && fizz = 0 )", FilterCondition::Not(Box::new(
FilterCondition::Or(vec![
    FilterCondition::And(vec![
        FilterCondition::ValueCondition { field: "foo".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned() },
        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::LessThan, second_field: "bar".to_owned() }
     ]),
     FilterCondition::And(vec![
        FilterCondition::FieldCondition { first_field: "foo".to_owned(), operator: ComparisonOperator::Equal, second_field: "bar".to_owned() },
        FilterCondition::ValueCondition { field: "fizz".to_owned(), operator: ComparisonOperator::Equal, value: "0 ".to_owned()}
    ]),
]))))]
fn test_rule_filter_expr_success(
    #[case] input: &str,
    #[case] expected_filter_cond: FilterCondition,
) {
    let parsed = rule::RuleFilterExprParser::new().parse(input);
    if parsed.is_err() {
        println!("{:?}", parsed.as_ref().unwrap_err());
    }
    assert!(parsed.is_ok());

    assert_eq!(parsed.unwrap(), expected_filter_cond);
}

#[rstest]
#[case("foo > 0.0")]
#[case("1 > foo")]
#[case("foo > -0.")]
#[case("(foo = 0 && foo < bar")]
fn test_rule_filter_expr_failure(#[case] input: &str) {
    let parsed = rule::RuleFilterExprParser::new().parse(input);
    assert!(parsed.is_err());
}
