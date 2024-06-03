use clap::ValueEnum;
use serde::Serialize;

pub(crate) mod dqdl;
pub mod pydeequ;
pub mod pyspark_class;
mod test_strings;

#[derive(Debug, ValueEnum, Serialize, Default, Clone)]
pub enum CompilationTarget {
    PyDeequ,
    Dqdl,
    PySparkClass,
    #[default]
    None,
}
