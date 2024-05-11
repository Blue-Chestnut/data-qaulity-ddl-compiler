use crate::model::rule_ext_config::RuleExtConfig;

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnRule {
    LikePattern {
        name: String,
        pattern: String,
        rule_ext_config: RuleExtConfig,
    },
    RegexPattern {
        name: String,
        pattern: String,
        rule_ext_config: RuleExtConfig,
    },
    ContainsValue {
        name: String,
        value: String,
        rule_ext_config: RuleExtConfig,
    },
}
