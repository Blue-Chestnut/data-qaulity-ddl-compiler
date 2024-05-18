use clap::ValueEnum;
use serde::Serialize;

pub(crate) mod dqdl;
pub mod pydeequ;
mod test_strings;

#[derive(Debug, ValueEnum, Serialize, Default, Clone)]
pub enum CompilationTarget {
    PyDeequ,
    #[warn(clippy::upper_case_acronyms)]
    DQDL,
    #[default]
    None,
}
