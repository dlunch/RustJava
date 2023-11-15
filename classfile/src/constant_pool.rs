use alloc::string::String;

use nom::{
    combinator::{flat_map, map},
    multi::length_count,
    number::complete::{be_u16, u8},
    IResult,
};
use nom_derive::{NomBE, Parse};

fn parse_utf8(data: &[u8]) -> IResult<&[u8], String> {
    map(length_count(be_u16, u8), |x| String::from_utf8(x.to_vec()).unwrap())(data)
}

#[derive(NomBE)]
pub struct ConstantClassInfo {
    pub name_index: u16,
}

#[derive(NomBE)]
pub struct ConstantStringInfo {
    pub string_index: u16,
}

#[derive(NomBE)]
pub struct ConstantReferenceInfo {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(NomBE)]
pub struct ConstantNameAndTypeInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(NomBE)]
#[nom(Selector = "u8")]
pub enum ConstantPoolItem {
    #[nom(Selector = "1")]
    Utf8(#[nom(Parse = "parse_utf8")] String),
    #[nom(Selector = "3")]
    Integer(u32),
    #[nom(Selector = "4")]
    Float(f32),
    #[nom(Selector = "5")]
    Long(u64),
    #[nom(Selector = "6")]
    Double(f64),
    #[nom(Selector = "7")]
    Class(ConstantClassInfo),
    #[nom(Selector = "8")]
    String(ConstantStringInfo),
    #[nom(Selector = "9")]
    Fieldref(ConstantReferenceInfo),
    #[nom(Selector = "10")]
    Methodref(ConstantReferenceInfo),
    #[nom(Selector = "11")]
    InstanceMethodref(ConstantReferenceInfo),
    #[nom(Selector = "12")]
    NameAndType(ConstantNameAndTypeInfo),
}

impl ConstantPoolItem {
    // TODO is there some proc_macro to generate this?
    pub fn parse_with_tag(data: &[u8]) -> IResult<&[u8], Self> {
        flat_map(u8, |x| move |i| Self::parse(i, x))(data)
    }

    pub fn utf8(&self) -> &str {
        if let ConstantPoolItem::Utf8(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn integer(&self) -> u32 {
        if let ConstantPoolItem::Integer(x) = self {
            *x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn float(&self) -> f32 {
        if let ConstantPoolItem::Float(x) = self {
            *x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn long(&self) -> u64 {
        if let ConstantPoolItem::Long(x) = self {
            *x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn double(&self) -> f64 {
        if let ConstantPoolItem::Double(x) = self {
            *x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn class(&self) -> &ConstantClassInfo {
        if let ConstantPoolItem::Class(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn string(&self) -> &ConstantStringInfo {
        if let ConstantPoolItem::String(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn fieldref(&self) -> &ConstantReferenceInfo {
        if let ConstantPoolItem::Fieldref(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn methodref(&self) -> &ConstantReferenceInfo {
        if let ConstantPoolItem::Methodref(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn instance_methodref(&self) -> &ConstantReferenceInfo {
        if let ConstantPoolItem::InstanceMethodref(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn name_and_type(&self) -> &ConstantNameAndTypeInfo {
        if let ConstantPoolItem::NameAndType(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }
}
