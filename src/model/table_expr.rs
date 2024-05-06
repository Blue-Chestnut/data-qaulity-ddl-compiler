pub struct TableDef {
    pub table_ref: String,
    pub columns: Vec<Box<ColumnDef>>,
    // table_level_rules: Vec<TableLevelRule>,
}

pub struct ColumnDef {
    pub name: String,
    pub data_type: String,
}

pub struct TableLevelRule {
    pub name: String,
}
