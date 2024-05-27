use crate::model::rule_ext_config::RuleExtConfig;
use crate::model::rule_traits::{ColumnValidationError, ValidColumnRule};
use crate::model::table_expr::{ColumnDef, DataType};
use valid_column_rule_derive::ValidColumnRule;

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

impl ValidColumnRule for LikePattern {
    fn validate_col_type(&self, column: &ColumnDef) -> Result<String, ColumnValidationError> {
        if column.data_type.class.is_string_like() {
            Ok(String::from("valid"))
        } else {
            Err(ColumnValidationError::InvalidType(format!(
                "Column {} is not a string-like type for like pattern rule",
                column.name
            )))
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
            threshold: threshold.unwrap_or(1.0),
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

impl ValidColumnRule for RegexPattern {
    fn validate_col_type(&self, column: &ColumnDef) -> Result<String, ColumnValidationError> {
        if column.data_type.class.is_string_like() {
            Ok(String::from("valid"))
        } else {
            Err(ColumnValidationError::InvalidType(format!(
                "Column {} is not a string-like type for contains regex rule",
                column.name
            )))
        }
    }
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
            threshold: threshold.unwrap_or(1.0),
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

impl ValidColumnRule for ContainsValue {
    fn validate_col_type(&self, column: &ColumnDef) -> Result<String, ColumnValidationError> {
        if column.data_type.class.is_string_like() {
            Ok(String::from("valid"))
        } else {
            Err(ColumnValidationError::InvalidType(format!(
                "Column {} is not a string-like type for contains value rule",
                column.name
            )))
        }
    }
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
            threshold: threshold.unwrap_or(1.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, ValidColumnRule)]
pub struct NonNull {
    pub name: String,
    pub rule_ext_config: RuleExtConfig,
    pub threshold: f32,
}

impl Default for NonNull {
    fn default() -> Self {
        Self {
            name: String::new(),
            rule_ext_config: RuleExtConfig::new_empty(),
            threshold: 1.0,
        }
    }
}

impl NonNull {
    pub fn new(
        name: Option<String>,
        rule_ext_config: Option<RuleExtConfig>,
        threshold: Option<f32>,
    ) -> Self {
        Self {
            name: name.unwrap_or_default(),
            rule_ext_config: rule_ext_config.unwrap_or_default(),
            threshold: threshold.unwrap_or(1.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NotEmpty {
    pub name: String,
    pub rule_ext_config: RuleExtConfig,
    pub threshold: f32,
}

impl ValidColumnRule for NotEmpty {
    fn validate_col_type(&self, column: &ColumnDef) -> Result<String, ColumnValidationError> {
        if column.data_type.class.is_string_like() {
            Ok(String::from("valid"))
        } else {
            Err(ColumnValidationError::InvalidType(format!(
                "Column {} is not a string-like type for not empty rule",
                column.name
            )))
        }
    }
}

impl Default for NotEmpty {
    fn default() -> Self {
        Self {
            name: String::new(),
            rule_ext_config: RuleExtConfig::new_empty(),
            threshold: 1.0,
        }
    }
}

impl NotEmpty {
    pub fn new(
        name: Option<String>,
        rule_ext_config: Option<RuleExtConfig>,
        threshold: Option<f32>,
    ) -> Self {
        Self {
            name: name.unwrap_or_default(),
            rule_ext_config: rule_ext_config.unwrap_or_default(),
            threshold: threshold.unwrap_or(1.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, ValidColumnRule)]
pub struct Uniqueness {
    pub name: String,
    pub rule_ext_config: RuleExtConfig,
}

impl Default for Uniqueness {
    fn default() -> Self {
        Self {
            name: String::new(),
            rule_ext_config: RuleExtConfig::new_empty(),
        }
    }
}

impl Uniqueness {
    pub fn new(name: Option<String>, rule_ext_config: Option<RuleExtConfig>) -> Self {
        Self {
            name: name.unwrap_or_default(),
            rule_ext_config: rule_ext_config.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, ValidColumnRule)]
pub struct IsType {
    pub name: String,
    pub data_type: DataType,
    pub rule_ext_config: RuleExtConfig,
}

impl Default for IsType {
    fn default() -> Self {
        Self {
            name: String::new(),
            data_type: DataType::new("Varchar", Some(3), None),
            rule_ext_config: RuleExtConfig::new_empty(),
        }
    }
}

impl IsType {
    pub fn new(
        name: Option<String>,
        data_type: DataType,
        rule_ext_config: Option<RuleExtConfig>,
    ) -> Self {
        Self {
            name: name.unwrap_or_default(),
            data_type,
            rule_ext_config: rule_ext_config.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ColumnRule {
    LikePattern(LikePattern),
    RegexPattern(RegexPattern),
    ContainsValue(ContainsValue),
    NonNull(NonNull),
    NotEmpty(NotEmpty),
    Uniqueness(Uniqueness),
    IsType(IsType),
}

impl ValidColumnRule for ColumnRule {
    fn validate_col_type(&self, column: &ColumnDef) -> Result<String, ColumnValidationError> {
        match self {
            ColumnRule::LikePattern(rule) => rule.validate_col_type(column),
            ColumnRule::RegexPattern(rule) => rule.validate_col_type(column),
            ColumnRule::ContainsValue(rule) => rule.validate_col_type(column),
            ColumnRule::NonNull(rule) => rule.validate_col_type(column),
            ColumnRule::NotEmpty(rule) => rule.validate_col_type(column),
            ColumnRule::Uniqueness(rule) => rule.validate_col_type(column),
            ColumnRule::IsType(rule) => rule.validate_col_type(column),
            // _ => Err(ColumnValidationError::RuleValidationNotImplemented(
            //     String::from("rule validation not implemented"),
            // )),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::model::column_rule::{
        ColumnRule, ContainsValue, LikePattern, RegexPattern, Uniqueness,
    };
    use crate::model::rule_traits::ValidColumnRule;
    use crate::model::table_expr::{ColumnDef, DataType};
    use rstest::rstest;

    #[rstest]
    #[case(ColumnRule::RegexPattern(RegexPattern::new(None, "".to_owned(), None, None)),
    ColumnDef::new(String::from("test"), DataType::new("Varchar", Some(3), None), false, false))]
    #[case(ColumnRule::LikePattern(LikePattern::new(None, "".to_owned(), None, None)),
    ColumnDef::new(String::from("test"), DataType::new("text", Some(3), None), false, false))]
    #[case(
        ColumnRule::Uniqueness(Uniqueness::new(None, None)),
        ColumnDef::new(
            String::from("test"),
            DataType::new("int", Some(3), None),
            false,
            false
        )
    )]
    #[case(ColumnRule::ContainsValue(ContainsValue::new(None, "".to_owned(), None, None)),
    ColumnDef::new(String::from("test"), DataType::new("LongText", None, None), false, false))]
    pub fn test_rule_type_validation_success(
        #[case] column_rule: ColumnRule,
        #[case] column: ColumnDef,
    ) {
        let is_valid = column_rule.validate_col_type(&column).is_ok();
        assert!(is_valid);
    }

    #[rstest]
    #[case(ColumnRule::RegexPattern(RegexPattern::new(None, "".to_owned(), None, None)),
    ColumnDef::new(String::from("test"), DataType::new("Float", Some(3), None), false, false))]
    #[case(ColumnRule::LikePattern(LikePattern::new(None, "".to_owned(), None, None)),
    ColumnDef::new(String::from("test"), DataType::new("Int", Some(3), None), false, false))]
    #[case(ColumnRule::ContainsValue(ContainsValue::new(None, "".to_owned(), None, None)),
    ColumnDef::new(String::from("test"), DataType::new("bool", None, None), false, false))]
    #[should_panic]
    pub fn test_rule_type_validation_failure(
        #[case] column_rule: ColumnRule,
        #[case] column: ColumnDef,
    ) {
        let is_valid = column_rule.validate_col_type(&column).is_ok();
        assert!(is_valid);
    }
}
