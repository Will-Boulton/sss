pub use array_like::ArrayLike;
pub use scalar::ScalarType;
pub mod array_like;
pub mod scalar;

mod enumeration;

mod structure;

pub trait FieldType {
    fn size_bytes(&self) -> usize;
}

#[derive(Debug)]
pub enum FieldType_ {
    Scalar(ScalarType),
    Vector(ArrayLike),
    Padding(usize),
}

impl FieldType for FieldType_ {
    fn size_bytes(&self) -> usize {
        match self {
            FieldType_::Scalar(st) => st.size_bytes(),
            FieldType_::Vector(vt) => vt.size_bytes(),
            FieldType_::Padding(len) => *len,
        }
    }
}
