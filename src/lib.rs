pub mod lexer;
pub mod parser;
pub mod source;
pub mod syntax;

#[cfg(test)]
mod test {

    use crate::syntax::*;
    #[test]
    fn main() {
        println!("Hello, world!");
        let my_struct = StructDeclarationSyntax {
            name: String::from("message_1"),
            id: Some(1usize),
            fields: vec![FieldDeclaration {
                name: String::from("field1"),
                field_type: FieldType::BuiltIn(BuiltInType::Integer {
                    size_bits: 16,
                    signedness: Signedness::Signed,
                    maybe_endianness: None,
                }),
                description: Some(String::from("the first field")),
            }],
            description: None,
        };
        println!("{:?}", my_struct)
    }
}
