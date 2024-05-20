use crate::data_types::FieldType;
use Signed::*;
#[derive(Debug)]
pub enum Signed {
    I8,
    I16,
    I32,
    I64,
}

impl FieldType for Signed {
    fn size_bytes(&self) -> usize {
        return match self {
            I8 => 1,
            I16 => 2,
            I32 => 4,
            I64 => 8,
        };
    }
}

impl Signed {
    pub fn try_parse(txt: &str) -> Option<Signed>
    {
        return match txt {
            "i8" => Some(I8),
            "i16" => Some(I16),
            "i32" => Some(I32),
            "i64" => Some(I64),
            _ => None
        }
    }
}