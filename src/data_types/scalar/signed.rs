use crate::data_types::FieldType;

pub enum Signed{
    I8,
    I16,
    I32,
    I64
}

impl FieldType for Signed {
    fn size_bytes(&self) -> usize {
        return match self  {
            Signed::I8 => 1,
            Signed::I16 => 2,
            Signed::I32 => 4,
            Signed::I64 => 8
        }
    }
}