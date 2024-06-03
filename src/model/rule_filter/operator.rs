use std::fmt::Display;

use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ComparisonOperator {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

impl Display for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::Equal => "=",
            ComparisonOperator::NotEqual => "!=",
            ComparisonOperator::GreaterThanOrEqual => ">=",
            ComparisonOperator::LessThanOrEqual => "<=",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(ComparisonOperator::GreaterThan, ">")]
    #[case(ComparisonOperator::LessThan, "<")]
    #[case(ComparisonOperator::Equal, "=")]
    #[case(ComparisonOperator::NotEqual, "!=")]
    #[case(ComparisonOperator::GreaterThanOrEqual, ">=")]
    #[case(ComparisonOperator::LessThanOrEqual, "<=")]
    fn test_to_string(#[case] operator: ComparisonOperator, #[case] expected_str: &str) {
        assert_eq!(operator.to_string(), expected_str);
    }
}

// pub enum RuleFilterBinaryOperators {
//     And,
//     Or,
//     Not,
// }
