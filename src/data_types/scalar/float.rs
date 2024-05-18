use crate::data_types::FieldType;

pub enum Float {
    F32,
    F64
}

impl FieldType for Float {
    fn size_bytes(&self) -> usize {
        match self {
            Float::F32 => 4,
            Float::F64 => 8
        }
    }
}