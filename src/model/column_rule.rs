use crate::model::rule_ext_config::RuleExtConfig;

#[derive(Clone, Debug, PartialEq)]
pub struct LikePattern {
    pub name: String,
    pub pattern: String,
    pub rule_ext_config: RuleExtConfig,
    pub threshold: f32,
}

impl Default for LikePattern {
    fn default() -> Self {
        Self {
            name: String::new(),
            pattern: String::new(),
            rule_ext_config: RuleExtConfig::new_empty(),
            threshold: 1.0,
        }
    }
}

impl LikePattern {
    pub fn new(
        name: Option<String>,
        pattern: String,
        rule_ext_config: Option<RuleExtConfig>,
        threshold: Option<f32>,
    ) -> Self {
        Self {
            name: name.unwrap_or_default(),
            pattern,
            rule_ext_config: rule_ext_config.unwrap_or_default(),
            threshold: threshold.unwrap_or_else(|| 1.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegexPattern {
    pub name: String,
    pub pattern: String,
    pub rule_ext_config: RuleExtConfig,
    pub threshold: f32,
}

impl Default for RegexPattern {
    fn default() -> Self {
        Self {
            name: String::new(),
            pattern: String::new(),
            rule_ext_config: RuleExtConfig::new_empty(),
            threshold: 1.0,
        }
    }
}

impl RegexPattern {
    pub fn new(
        name: Option<String>,
        pattern: String,
        rule_ext_config: Option<RuleExtConfig>,
        threshold: Option<f32>,
    ) -> Self {
        Self {
            name: name.unwrap_or_default(),
            pattern,
            rule_ext_config: rule_ext_config.unwrap_or_default(),
            threshold: threshold.unwrap_or_else(|| 1.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContainsValue {
    pub name: String,
    pub value: String,
    pub rule_ext_config: RuleExtConfig,
    pub threshold: f32,
}

impl Default for ContainsValue {
    fn default() -> Self {
        Self {
            name: String::new(),
            value: String::new(),
            rule_ext_config: RuleExtConfig::new_empty(),
            threshold: 1.0,
        }
    }
}

impl ContainsValue {
    pub fn new(
        name: Option<String>,
        value: String,
        rule_ext_config: Option<RuleExtConfig>,
        threshold: Option<f32>,
    ) -> Self {
        Self {
            name: name.unwrap_or_default(),
            value,
            rule_ext_config: rule_ext_config.unwrap_or_default(),
            threshold: threshold.unwrap_or_else(|| 1.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnRule {
    LikePattern(LikePattern),
    RegexPattern(RegexPattern),
    ContainsValue(ContainsValue),
}
