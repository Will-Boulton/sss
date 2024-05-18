use scalar::ScalarType;
use array_like::ArrayLike;
pub mod scalar;
pub mod array_like;

mod enumeration;

mod structure;

trait FieldType {
    fn size_bytes(&self) -> usize;
}


enum FieldType_ {
    Scalar(ScalarType),
    Vector(ArrayLike),
    Padding(usize)
}

impl FieldType for FieldType_ {
    fn size_bytes(&self) -> usize {
        match self {
            FieldType_::Scalar(st) => st.size_bytes(),
            FieldType_::Vector(vt) => vt.size_bytes(),
            FieldType_::Padding(len) => *len
        }
    }
}