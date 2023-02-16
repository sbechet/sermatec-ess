pub enum FieldType {
    Bit(bool),
    Int(i32),
    BitRange(u16),
    // Bytes(Vec<u8>),
    Hex(u16),
    Long(i32),
    OnePosition(Vec<u8>),
    Preserve(Vec<u8>),
    String(String),
    UInt(u32),
    Repeat(Vec<FieldType>), // Special type for repeat field
}

impl std::fmt::Debug for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match &self {
            FieldType::Bit(b) => format!("{}", b),
            FieldType::Int(i) => format!("{}", i),
            FieldType::BitRange(b) => format!("{}", b),
            FieldType::Hex(u) => format!("{}", u),
            FieldType::OnePosition(i) => format!("{:x?}", i),
            FieldType::Preserve(i) => format!("{:x?}", i),
            FieldType::Long(i) => format!("{}", i),
            FieldType::String(s) => format!("{}", s),
            FieldType::UInt(u) => format!("{}", u),
            FieldType::Repeat(v) => format!("{:x?}", v),
        };
        write!(f, "{}", t)
    }
}