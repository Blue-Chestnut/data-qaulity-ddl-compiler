#[derive(Clone, Debug, PartialEq, Default)]
#[allow(dead_code)]
pub enum DataClass {
    // Unknown,
    #[default]
    Unknown,
    // Boolean Types
    Bit,
    Bool,
    // String Types
    Char,
    VarChar,
    Binary,
    VarBinary,
    TinyBlob,
    TinyText,
    Text,
    Blob,
    MediumText,
    MediumBlob,
    LongText,
    LongBlob,
    // Enum,
    // Set,
    // Numeric Types
    TinyInt,
    SmallInt,
    MediumInt,
    Int,
    Integer,
    BigInt,
    Float,
    Double,
    DoublePrecision,
    Decimal,
    Dec,
    // Dates
    Date,
    Time,
    DateTime,
    Timestamp,
    Year,
}

impl std::fmt::Display for DataClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DataClass {
    pub fn is_string_like(&self) -> bool {
        matches!(
            self,
            DataClass::Char
                | DataClass::VarChar
                | DataClass::Binary
                | DataClass::VarBinary
                | DataClass::TinyBlob
                | DataClass::TinyText
                | DataClass::Text
                | DataClass::Blob
                | DataClass::MediumText
                | DataClass::MediumBlob
                | DataClass::LongText
                | DataClass::LongBlob
        )
    }

    pub fn is_boolean_like(&self) -> bool {
        matches!(self, DataClass::Bit | DataClass::Bool)
    }

    pub fn is_fraction_like(&self) -> bool {
        matches!(
            self,
            DataClass::Float
                | DataClass::Double
                | DataClass::DoublePrecision
                | DataClass::Decimal
                | DataClass::Dec
        )
    }

    pub fn is_numeric_like(&self) -> bool {
        matches!(
            self,
            DataClass::TinyInt
                | DataClass::SmallInt
                | DataClass::MediumInt
                | DataClass::Int
                | DataClass::Integer
                | DataClass::BigInt
                | DataClass::Float
                | DataClass::Double
                | DataClass::DoublePrecision
                | DataClass::Decimal
                | DataClass::Dec
        )
    }

    pub fn is_date_like(&self) -> bool {
        matches!(
            self,
            DataClass::Date
                | DataClass::Time
                | DataClass::DateTime
                | DataClass::Timestamp
                | DataClass::Year
        )
    }
}
