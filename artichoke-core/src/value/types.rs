use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Rust {
    Bool,
    Bytes,
    Float,
    Map,
    Object,
    SignedInt,
    String,
    UnsignedInt,
    Vec,
}

impl fmt::Display for Rust {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rust ")?;
        match self {
            Rust::Bool => write!(f, "bool"),
            Rust::Bytes => write!(f, "&[u8]"),
            Rust::Float => write!(f, "f64"),
            Rust::Map => write!(f, "HashMap"),
            Rust::Object => write!(f, "struct"),
            Rust::SignedInt => write!(f, "i64"),
            Rust::String => write!(f, "String"),
            Rust::Vec => write!(f, "Vec"),
            Rust::UnsignedInt => write!(f, "usize"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Ruby {
    Array,
    Bool,
    Class,
    CPointer,
    Data,
    Exception,
    Fiber,
    Fixnum,
    Float,
    Hash,
    InlineStruct,
    Module,
    Nil,
    Object,
    Proc,
    Range,
    SingletonClass,
    String,
    Symbol,
    Unreachable,
}

impl Ruby {
    pub fn class_name(&self) -> String {
        match self {
            Ruby::Array => "Array",
            Ruby::Bool => "Boolean",
            Ruby::Class => "Class",
            Ruby::CPointer => "C Pointer",
            Ruby::Data => "Rust-backed Ruby instance",
            Ruby::Exception => "Exception",
            Ruby::Fiber => "Fiber",
            Ruby::Fixnum => "Fixnum",
            Ruby::Float => "Float",
            Ruby::Hash => "Hash",
            Ruby::InlineStruct => "Inline Struct",
            Ruby::Module => "Module",
            Ruby::Nil => "NilClass",
            Ruby::Object => "Object",
            Ruby::Proc => "Proc",
            Ruby::Range => "Range",
            Ruby::SingletonClass => "Singleton (anonymous) class",
            Ruby::String => "String",
            Ruby::Symbol => "Symbol",
            Ruby::Unreachable => "internal and unreachable",
        }
        .to_owned()
    }
}

impl fmt::Display for Ruby {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ruby {}", self.class_name())
    }
}
