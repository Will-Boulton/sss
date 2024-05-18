use crate::data_types::FieldType;

pub enum Unsigned{
    U8,
    U16,
    U32,
    U64
}

impl FieldType for Unsigned {
    fn size_bytes(&self) -> usize {
        return match self  {
            Unsigned::U8 => 1,
            Unsigned::U16 => 2,
            Unsigned::U32 => 4,
            Unsigned::U64 => 8
        }
    }
}