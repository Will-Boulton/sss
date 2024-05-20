use crate::data_types::{FieldType, FieldType_};
pub use byte_size::ByteSize;
pub use float::Float;
pub use signed::Signed;
pub use unsigned::Unsigned;

mod byte_size;
mod float;
mod signed;
mod unsigned;

#[derive(Debug)]
pub enum ScalarType {
    Unsigned(Unsigned),
    Signed(Signed),
    Float(Float),
    ByteSized(ByteSize),
}

impl FieldType for ScalarType {
    fn size_bytes(&self) -> usize {
        match self {
            ScalarType::Unsigned(u) => u.size_bytes(),
            ScalarType::Signed(u) => u.size_bytes(),
            ScalarType::Float(u) => u.size_bytes(),
            ScalarType::ByteSized(u) => u.size_bytes(),
        }
    }
}

impl ScalarType {
    pub fn try_parse(txt: &str) -> Option<ScalarType>
    {
        Unsigned::try_parse(txt).map(|x|ScalarType::Unsigned(x))
            .or_else(||Signed::try_parse(txt).map(|x|ScalarType::Signed(x))
            .or_else(||Float::try_parse(txt).map(|x|ScalarType::Float(x)))
            .or_else(||ByteSize::try_parse(txt).map(|x|ScalarType::ByteSized(x))))
    }
}