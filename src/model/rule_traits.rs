use crate::model::table_expr::ColumnDef;

#[derive(Debug)]
pub enum ColumnValidationError {
    InvalidType(String),
    // RuleValidationNotImplemented(String),
}

pub trait ValidColumnRule {
    fn validate_col_type(&self, column: &ColumnDef) -> Result<String, ColumnValidationError>;
}
