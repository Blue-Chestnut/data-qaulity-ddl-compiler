use filter::ColumnRuleFilter;

pub mod condition;
pub mod filter;
pub mod operator;

/// Utility that combines identical filters to remove any duplication
/// the duplication detection is based on the string of the parsed filter.
/// This is not equivalent to logical equivalence:
/// !(A && B) == !(A || B) logically equivalent but this utility will say they are not
///
/// The utility currently works by sorting the string version of filters and gruping identical ones.
pub fn combine_itentical_filters(
    column_rule_filters: Vec<ColumnRuleFilter>,
) -> Vec<ColumnRuleFilter> {
    let mut stringified_filters = column_rule_filters
        .iter()
        .map(|f| (f.to_string(), f.to_owned()))
        .collect::<Vec<(String, ColumnRuleFilter)>>();
    stringified_filters.sort_by(|a, b| a.0.cmp(&b.0));

    let mut combined_filters: Vec<ColumnRuleFilter> = vec![];
    combined_filters.push(stringified_filters[0].1.to_owned());
    let mut last_filter_string = stringified_filters[0].0.clone();
    let mut last_index = 0;

    for (i, (name, filter)) in stringified_filters.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let last_filter = combined_filters.last().unwrap();

        if last_filter_string == *name {
            let mut rules = last_filter.rules.to_owned();
            rules.extend(filter.rules.to_owned());

            combined_filters[last_index] = ColumnRuleFilter {
                filter_string: last_filter.filter_string.to_owned(),
                rules,
                filter_condition: last_filter.filter_condition.to_owned(),
            };
            // last_filter.rules.extend(filter.rules)
        } else {
            combined_filters.push(filter.to_owned());
            last_filter_string = name.clone();
            last_index += 1;
        }
    }

    return combined_filters;
}

#[cfg(test)]
mod tests {
    use filter::FilterCondition;
    use operator::ComparisonOperator;
    use rstest::rstest;

    use crate::model::column_rule::{ColumnRule, NotEmpty};

    use super::*;

    #[rstest]
    #[case(
        vec![
            ColumnRuleFilter {
                filter_string: Some("".to_owned()),
                rules: vec![ColumnRule::NotEmpty(NotEmpty{name: "a".to_owned(), ..Default::default()})],
                filter_condition: None,
            },
            ColumnRuleFilter {
                filter_string: Some("".to_owned()),
                rules: vec![ColumnRule::NotEmpty(NotEmpty{name: "a".to_owned(), ..Default::default()})],
                filter_condition: Some(FilterCondition::FieldCondition { first_field: "Price".to_owned(), 
                operator: ComparisonOperator::GreaterThan, second_field: "Id".to_owned() }),
            },
            ColumnRuleFilter {
                filter_string: Some("".to_owned()),
                rules: vec![ColumnRule::NotEmpty(NotEmpty{name: "b".to_owned(), ..Default::default()})],
                filter_condition: Some(FilterCondition::FieldCondition { first_field: "Price".to_owned(), 
                operator: ComparisonOperator::GreaterThan, second_field: "Id".to_owned() }),
            },
            ColumnRuleFilter {
                filter_string: Some("".to_owned()),
                rules: vec![ColumnRule::NotEmpty(NotEmpty{name: "c".to_owned(), ..Default::default()})],
                filter_condition: Some(FilterCondition::FieldCondition { first_field: "Price".to_owned(), 
                operator: ComparisonOperator::GreaterThan, second_field: "Id".to_owned() }),
            },
            ColumnRuleFilter {
                filter_string: Some("".to_owned()),
                rules: vec![ColumnRule::NotEmpty(NotEmpty{name: "b".to_owned(), ..Default::default()})],
                filter_condition: None,
            },
            ColumnRuleFilter {
                filter_string: Some("".to_owned()),
                rules: vec![ColumnRule::NotEmpty(NotEmpty{name: "c".to_owned(), ..Default::default()})],
                filter_condition: None,
            },
        ],
        vec![
            ColumnRuleFilter {
                filter_string: Some("".to_owned()),
                rules: vec![
                    ColumnRule::NotEmpty(NotEmpty{name: "a".to_owned(), ..Default::default()}),
                    ColumnRule::NotEmpty(NotEmpty{name: "b".to_owned(), ..Default::default()}),
                    ColumnRule::NotEmpty(NotEmpty{name: "c".to_owned(), ..Default::default()})
                    ],
                filter_condition: None,
            },
            ColumnRuleFilter {
                filter_string: Some("".to_owned()),
                rules: vec![
                    ColumnRule::NotEmpty(NotEmpty{name: "a".to_owned(), ..Default::default()}),
                    ColumnRule::NotEmpty(NotEmpty{name: "b".to_owned(), ..Default::default()}),
                    ColumnRule::NotEmpty(NotEmpty{name: "c".to_owned(), ..Default::default()})
                    ],
                filter_condition: Some(FilterCondition::FieldCondition { first_field: "Price".to_owned(), 
                operator: ComparisonOperator::GreaterThan, second_field: "Id".to_owned() }),
            },
        ],
    )]
    fn test_combine_filters(
        #[case] input_filters: Vec<ColumnRuleFilter>,
        #[case] expected_filters: Vec<ColumnRuleFilter>,
    ) {
        let actual_filters = combine_itentical_filters(input_filters);
        assert_eq!(actual_filters, expected_filters);
    }
}
