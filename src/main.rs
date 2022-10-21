pub mod types;
pub mod lexer;
pub mod error;

use types::*;
use error::*;

fn main() {
    let ptr = Type::Complex(ComplexType::Pointer(Pointer::new(8, 8, 2, SimpleType::Char)));
    println!("{}", ptr);

    let valid_arr = Type::Complex(ComplexType::Array(Array::new(0, true, SimpleType::String)));
    println!("{}", valid_arr);

    let err = Error::new(1, 1, ErrorType::TypeError, "cannot coerce type `Array<u8>` to `*void`!");
    println!("{}", err);
}
