
pub enum FieldType {
    // Bit(bool),
    Int(f64),
    // BitRange(Vec<bool>),
    // Bytes(Vec<u8>),
    Hex(u16),
    Long(f64),
    // OnePosiiton(String),
    // Preserve(Vec<u8>),
    String(String),
    UInt(f64),
}

impl std::fmt::Debug for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match &self {
            FieldType::Int(i) => format!("{}", i),
            FieldType::Hex(u) => format!("{}", u),
            FieldType::Long(i) => format!("{}", i),
            FieldType::String(s) => format!("{}", s),
            FieldType::UInt(u) => format!("{}", u),
        };
        write!(f, "{}", t)
    }
}