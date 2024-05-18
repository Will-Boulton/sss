use crate::data_types::{ScalarType, FieldType};

pub enum ArrayLike {
    FixedArray{scalar: ScalarType, length: usize },
    AsciiString{length: usize},
    Bytes{length: usize}
}


impl FieldType for ArrayLike {
    fn size_bytes(&self) -> usize {
        match self {
            ArrayLike::FixedArray { scalar, length} => {
                length * scalar.size_bytes()
            }
            ArrayLike::AsciiString { length } => {
                *length
            }
            ArrayLike::Bytes { length } => {
                *length
            }
        }
    }
}