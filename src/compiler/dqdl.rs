use crate::model::column_rule::ColumnRule;
use crate::model::table_expr::TableDef;

pub fn compile_column_rule(
    column_rule: ColumnRule,
    table_name: String,
    column_name: String,
) -> String {
    match column_rule {
        ColumnRule::RegexPattern(rule) => {
            format!(
                "CustomSql \"select count() from {} where {} like '{}' \"",
                table_name, column_name, rule.pattern
            )
        }
        ColumnRule::LikePattern(rule) => {
            format!(
                "CustomSql \"select count() from {} where {} like '{}' \"",
                table_name, column_name, rule.pattern
            )
        }
        ColumnRule::ContainsValue(rule) => {
            format!(
                "CustomSql \"select count() from {} where {} like '%{}%' \"",
                table_name, column_name, rule.value
            )
        }
        ColumnRule::Uniqueness(_) => {
            format!("IsPrimaryKey \"{}\"", column_name)
        }
        ColumnRule::NotEmpty(_) => {
            format!("ColumnLength \"{}\" > 0", column_name)
        }
        ColumnRule::NonNull(_) => {
            format!("IsComplete \"{}\"", column_name)
        }
        ColumnRule::IsType(rule) => {
            format!(
                "ColumnDataType \"{}\" = \"{}\"",
                column_name, rule.data_type.class
            )
        } // _ => unimplemented!("DQDL has no implementation of rule: {:?}", column_rule)
    }
}

pub fn compile(table_def: TableDef) -> String {
    let mut compiled = String::new();
    for column_def in table_def.columns {
        for filter in column_def.rules {
            if filter.filter_string.is_some() {
                log::warn!("custom filters are not supported for DQDL at the moment!")
            }

            for rule in filter.rules {
                let compiled_rule = compile_column_rule(
                    rule,
                    table_def.table_ref.to_string().clone(),
                    column_def.name.clone(),
                );
                compiled.push_str(&compiled_rule);
                compiled.push_str(",\n");
            }
        }
    }
    compiled
}

#[cfg(test)]
mod tests {
    use crate::model::column_rule::{
        ColumnRule, ContainsValue, IsType, LikePattern, NonNull, NotEmpty, RegexPattern, Uniqueness,
    };
    use crate::model::rule_filter::filter::ColumnRuleFilter;
    use crate::model::table_expr::{ColumnDef, DataType, TableDef, TableRef};
    use rstest::rstest;

    use super::compile;

    #[rstest]
    #[case(
        ColumnRule::Uniqueness(Uniqueness::new(None, None)),
        "Test",
        "Id",
        "IsPrimaryKey \"Id\""
    )]
    #[case(
        ColumnRule::IsType(IsType::new(None, DataType::new("Int", Some(3), None), None)),
        "Test",
        "Id",
        "ColumnDataType \"Id\" = \"Int\""
    )]
    #[case(
        ColumnRule::NonNull(NonNull::new(None, None, None)),
        "Test",
        "Id",
        "IsComplete \"Id\""
    )]
    #[case(
        ColumnRule::NotEmpty(NotEmpty::new(None, None, None)),
        "Test",
        "Id",
        "ColumnLength \"Id\" > 0"
    )]
    #[case(ColumnRule::ContainsValue(ContainsValue::new(None, "test".to_owned(), None, None)), "Test", "Id", "CustomSql \"select count() from Test where Id like '%test%' \"")]
    #[case(ColumnRule::LikePattern(LikePattern::new(None, "test".to_owned(), None, None)), "Test", "Id", "CustomSql \"select count() from Test where Id like 'test' \"")]
    #[case(ColumnRule::RegexPattern(RegexPattern::new(None, "test".to_owned(), None, None)), "Test", "Id", "CustomSql \"select count() from Test where Id like 'test' \"")]
    pub fn compile_column_rule_test(
        #[case] column_rule: ColumnRule,
        #[case] table_name: &str,
        #[case] column_name: &str,
        #[case] expected: &str,
    ) {
        let actual =
            super::compile_column_rule(column_rule, table_name.to_owned(), column_name.to_owned());
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(TableDef {table_ref: TableRef::new("Test", None, None), columns: vec![]}, "")]
    #[case(TableDef {table_ref: TableRef::new("Test", None, None), columns: vec![
        ColumnDef::new("Id".to_owned(), DataType::new("INT", Some(3), None), false, false)
    ]}, "ColumnDataType \"Id\" = \"Int\",\n")]
    #[case(TableDef {table_ref: TableRef::new("Test", None, None), columns: vec![
        ColumnDef {name: "Id".to_owned(), data_type: DataType::new("INT", Some(3), None), not_null: false, primary_key: false, rules: 
        vec![ColumnRuleFilter::new(None, vec![
            ColumnRule::Uniqueness(Uniqueness::new(None, None)),
            ColumnRule::NonNull(NonNull::new(None, None, None)),
            ColumnRule::NotEmpty(NotEmpty::new(None, None, None)),
            ColumnRule::ContainsValue(ContainsValue::new(None, "test".to_owned(), None, None)),
        ])]}
    ]}, "IsPrimaryKey \"Id\",\nIsComplete \"Id\",\nColumnLength \"Id\" > 0,\nCustomSql \"select count() from Test where Id like '%test%' \",\n")]
    pub fn compile_test(#[case] table_def: TableDef, #[case] expected: &str) {
        let compiled = compile(table_def);
        assert_eq!(compiled, expected);
    }
}
