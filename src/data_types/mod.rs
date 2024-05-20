use array_like::ArrayLike;
use scalar::ScalarType;
pub mod array_like;
pub mod scalar;

mod enumeration;

mod structure;

trait FieldType {
    fn size_bytes(&self) -> usize;
}

#[derive(Debug)]
pub enum FieldType_ {
    Scalar(ScalarType),
    Vector(ArrayLike),
    Padding(usize),
}

impl FieldType_ {
    pub fn try_parse(txt: &str) -> Option<FieldType_>
    {
        ScalarType::try_parse(txt).map(|x|FieldType_::Scalar(x))
            .or_else(||ArrayLike::try_parse(txt).map(|x|FieldType_::Vector(x)))
            .or_else(||txt.parse::<usize>().ok().map(|s|FieldType_::Padding(s)))
    }
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
