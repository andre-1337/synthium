use std::{ fmt::{ Display, Formatter, Result } };

// placeholder(s) for the time being
pub trait PointerInfo {
    fn get_align(&self) -> usize;
    fn get_offset(&self) -> usize;
    fn get_references(&self) -> usize;
    fn get_base_type(&self) -> SimpleType;
}

pub trait ArrayInfo {
    fn get_size(&self) -> usize;
    fn get_base_type(&self) -> SimpleType;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SignedIntegerTypes {
    Int8,
    Int16,
    Int32,
    Int64,
}

impl Display for SignedIntegerTypes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Int8 => write!(f, "i8"),
            Self::Int16 => write!(f, "i16"),
            Self::Int32 => write!(f, "i32"),
            Self::Int64 => write!(f, "i64"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UnsignedIntegerTypes {
    Uint8,
    Uint16,
    Uint32,
    Uint64,
}

impl Display for UnsignedIntegerTypes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Uint8 => write!(f, "u8"),
            Self::Uint16 => write!(f, "u16"),
            Self::Uint32 => write!(f, "u32"),
            Self::Uint64 => write!(f, "u64"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FloatingPointTypes {
    FP16,
    FP32,
    FP64,
}

impl Display for FloatingPointTypes {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::FP16 => write!(f, "f16"),
            Self::FP32 => write!(f, "f32"),
            Self::FP64 => write!(f, "f64"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SignedInteger {
    pub base_type: SignedIntegerTypes,
    pub value: i64,
}

impl SignedInteger {
    pub fn new(base_type: SignedIntegerTypes, value: i64) -> Self {
        Self {
            base_type,
            value,
        }
    }
}

impl Display for SignedInteger {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({})", self.base_type, self.value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnsignedInteger {
    pub base_type: UnsignedIntegerTypes,
    pub value: u64,
}

impl UnsignedInteger {
    pub fn new(base_type: UnsignedIntegerTypes, value: u64) -> Self {
        Self {
            base_type,
            value,
        }
    }
}

impl Display for UnsignedInteger {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({})", self.base_type, self.value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct FloatingPoint {
    pub base_type: FloatingPointTypes,
    pub value: f64,
}

impl FloatingPoint {
    pub fn new(base_type: FloatingPointTypes, value: f64) -> Self {
        Self {
            base_type,
            value,
        }
    }
}

impl Display for FloatingPoint {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({})", self.base_type, self.value)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UserDefinedType {
    pub file: &'static str,
    pub name: &'static str,
}

impl UserDefinedType {
    pub fn new(file: &'static str, name: &'static str) -> Self {
        Self {
            file,
            name,
        }
    }
}

impl Display for UserDefinedType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}.{}", self.file, self.name)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pointer {
    pub alignment: usize,
    pub offset: usize,
    pub references: usize,
    pub base_type: SimpleType,
}

impl Pointer {
    pub fn new(alignment: usize, offset: usize, references: usize, base_type: SimpleType) -> Self {
        Self {
            alignment,
            offset,
            references,
            base_type,
        }
    }
}

impl Display for Pointer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}{}", "*".repeat(self.references), self.base_type)
    }
}

impl PointerInfo for Pointer {
    fn get_align(&self) -> usize {
        self.alignment
    }

    fn get_offset(&self) -> usize {
        self.offset
    }

    fn get_references(&self) -> usize {
        self.references
    }

    fn get_base_type(&self) -> SimpleType {
        self.base_type
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Array {
    pub size: usize,
    pub is_runtime_sized: bool,
    pub base_type: SimpleType,
}

impl Array {
    pub fn new(size: usize, is_runtime_sized: bool, base_type: SimpleType) -> Self {
        match base_type {
            SimpleType::Int(_) | SimpleType::Uint(_) | SimpleType::Fp(_) | SimpleType::Char | SimpleType::Bool | SimpleType::String | SimpleType::UserType(_) => {}
            SimpleType::Varargs => panic!("array cannot have type '...'"),
            SimpleType::Void => panic!("array cannot have type 'void'")
        }

        Self {
            size,
            is_runtime_sized,
            base_type,
        }
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let res: String;

        if self.is_runtime_sized {
            res = format!("[?]{}", self.base_type).to_owned();
        } else {
            res = format!("[{}]{}", self.size, self.base_type).to_owned();
        }

        write!(f, "{}", res)
    }
}

impl ArrayInfo for Array {
    fn get_size(&self) -> usize {
        self.size
    }

    fn get_base_type(&self) -> SimpleType {
        self.base_type
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SimpleType {
    Int(SignedInteger),
    Uint(UnsignedInteger),
    Fp(FloatingPoint),
    Char,
    Bool,
    String,
    Void,
    Varargs,
    UserType(UserDefinedType),
}

impl Display for SimpleType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Int(v) => write!(f, "{}", v),
            Self::Uint(v) => write!(f, "{}", v),
            Self::Fp(v) => write!(f, "{}", v),
            Self::Char => write!(f, "char"),
            Self::Bool => write!(f, "bool"),
            Self::String => write!(f, "string"),
            Self::Void => write!(f, "void"),
            Self::Varargs => write!(f, "..."),
            Self::UserType(v) => write!(f, "{}", v)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ComplexType {
    Pointer(Pointer),
    Array(Array),
}

impl Display for ComplexType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Pointer(pointer) => write!(f, "{}", pointer),
            Self::Array(array) => write!(f, "{}", array),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Type {
    Simple(SimpleType),
    Complex(ComplexType),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Simple(simple) => write!(f, "{}", simple),
            Self::Complex(complex) => write!(f, "{}", complex),
        }
    }
}

pub struct Typechecker;

impl Typechecker {
    fn coerce_types(type1: Type, type2: Type) -> bool {
        match type1 {
            Type::Simple(ty) => {
                match ty {
                    SimpleType::Int(_v) => todo!(),

                    SimpleType::Uint(_v) => todo!(),

                    SimpleType::Fp(_v) => todo!(),

                    SimpleType::Char => todo!(),

                    SimpleType::Bool => todo!(),

                    SimpleType::String => todo!(),

                    SimpleType::Void => {
                        panic!("cannot coerce a type to 'void'")
                    },

                    SimpleType::Varargs => {
                        panic!("cannot coerce a type to a vararg")
                    },

                    SimpleType::UserType(_) => {
                        panic!("coercing user defined types is not supported by the compiler yet")
                    },
                }
            },

            Type::Complex(ty) => {
                match ty {
                    ComplexType::Pointer(_ptr) => todo!(),

                    ComplexType::Array(arr) => {
                        match arr.base_type {
                            SimpleType::Uint(v) => {
                                match type2 {
                                    Type::Simple(ty) => {
                                        match ty {
                                            SimpleType::String => {
                                                if v.base_type == UnsignedIntegerTypes::Uint8 {
                                                    true
                                                } else {
                                                    false
                                                }
                                            },

                                            _ => panic!("cannot coerce type x to y")
                                        }
                                    },

                                    Type::Complex(ty) => {
                                        match ty {
                                            ComplexType::Pointer(_ptr) => todo!(),
                                            ComplexType::Array(_arr) => todo!(),
                                        }
                                    },
                                }
                            },

                            _ => panic!("cannot coerce this type yet"),
                        }
                    },
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::*;

    #[test]
    fn coerce_u8_arr_to_str() {
        let u8_arr = Type::Complex(ComplexType::Array(Array::new(0, true, SimpleType::Uint(UnsignedInteger::new(UnsignedIntegerTypes::Uint8, 0)))));
        let str = Type::Simple(SimpleType::String);

        assert!(Typechecker::coerce_types(u8_arr, str) == true);
    }
}
