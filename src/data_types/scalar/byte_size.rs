use crate::data_types::FieldType;
use ByteSize::*;
#[derive(Debug)]
pub enum ByteSize {
    Byte,
    Char,
    Ascii,
}

impl FieldType for ByteSize {
    fn size_bytes(&self) -> usize {
        1
    }
}

impl ByteSize {
    pub fn try_parse(txt: &str) -> Option<ByteSize> {
        return match txt {
            "byte" => Some(Byte),
            "char" => Some(Char),
            "ascii" => Some(Ascii),
            _ => None
        }
    }
}