use crate::data_types::{FieldType};

pub enum ByteSize {
    Byte,
    Char,
    Ascii
}

impl FieldType for ByteSize {
    fn size_bytes(&self) -> usize {
        1
    }
}