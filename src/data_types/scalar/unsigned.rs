use crate::data_types::FieldType;
use Unsigned::*;
#[derive(Debug)]
pub enum Unsigned {
    U8,
    U16,
    U32,
    U64,
}

impl FieldType for Unsigned {
    fn size_bytes(&self) -> usize {
        return match self {
            U8 => 1,
            U16 => 2,
            U32 => 4,
            U64 => 8,
        };
    }
}

impl Unsigned {
    pub fn try_parse(txt: &str) -> Option<Unsigned> {
        return match txt {
            "u8" => Some(U8),
            "u16" => Some(U16),
            "u32" => Some(U32),
            "u64" => Some(U64),
            _ => None,
        };
    }
}
