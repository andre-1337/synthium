pub mod types;

use types::*;

fn main() {
    let ptr = Type::Complex(ComplexType::Pointer(Pointer::new(8, 8, 2, SimpleType::Char)));
    println!("{}", ptr);

    let valid_arr = Type::Complex(ComplexType::Array(Array::new(0, true, SimpleType::String)));
    println!("{}", valid_arr);
}
