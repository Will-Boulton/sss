use crate::syntax::{BuiltInType, CustomType};

#[derive(Debug)]
pub enum FieldType {
    BuiltIn(BuiltInType),
    Custom(CustomType),
}
