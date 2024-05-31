use crate::model::rule_filter::operator::ComparisonOperator;

pub struct RuleFilter {
    condition: Condition,
}

pub enum Condition {
    ValueCondition {
        field: String,
        operator: ComparisonOperator,
        value: String,
    },
    FieldCondition {
        first_field: String,
        operator: ComparisonOperator,
        second_field: String,
    },
}
