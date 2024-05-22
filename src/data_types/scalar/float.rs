use crate::data_types::scalar::Float::*;
use crate::data_types::FieldType;

#[derive(Debug)]
pub enum Float {
    F32,
    F64,
}

impl FieldType for Float {
    fn size_bytes(&self) -> usize {
        match self {
            F32 => 4,
            F64 => 8,
        }
    }
}

impl Float {
    pub fn try_parse(txt: &str) -> Option<Float> {
        return match txt {
            "f32" => Some(F32),
            "f64" => Some(F64),
            _ => None,
        };
    }
}
