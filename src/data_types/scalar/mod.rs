pub use unsigned::Unsigned;
pub use signed::Signed;
pub use float::Float;
pub use byte_size::ByteSize;
use crate::data_types::FieldType;

mod unsigned;
mod signed;
mod float;
mod byte_size;


pub enum ScalarType {
    Unsigned(Unsigned),
    Signed(Signed),
    Float(Float),
    ByteSized(ByteSize)
}

impl FieldType for ScalarType {
    fn size_bytes(&self) -> usize {
        match self {
            ScalarType::Unsigned(u) => u.size_bytes(),
            ScalarType::Signed(u) => u.size_bytes(),
            ScalarType::Float(u) => u.size_bytes(),
            ScalarType::ByteSized(u) => u.size_bytes()
        }
    }
}