use alloc::string::{String, ToString};

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
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ConstantClassInfo {
    pub name_index: u16,
}

#[derive(NomBE)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ConstantStringInfo {
    pub string_index: u16,
}

#[derive(NomBE)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ConstantReferenceInfo {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(NomBE)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ConstantNameAndTypeInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(NomBE)]
#[cfg_attr(debug_assertions, derive(Debug))]
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

    pub fn class(&self) -> &ConstantClassInfo {
        if let ConstantPoolItem::Class(x) = self {
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

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum ValueConstant {
    Integer(u32),
    Float(f32),
    Long(u64),
    Double(f64),
    String(String),
    Class(String),
    Method(ReferenceConstant),
    Field(ReferenceConstant),
}

impl ValueConstant {
    pub fn from_constant_pool(constant_pool: &[ConstantPoolItem], index: usize) -> Self {
        match &constant_pool[index - 1] {
            ConstantPoolItem::Integer(x) => Self::Integer(*x),
            ConstantPoolItem::Float(x) => Self::Float(*x),
            ConstantPoolItem::Long(x) => Self::Long(*x),
            ConstantPoolItem::Double(x) => Self::Double(*x),
            ConstantPoolItem::String(x) => Self::String(constant_pool[x.string_index as usize - 1].utf8().to_string()),
            ConstantPoolItem::Class(x) => Self::Class(constant_pool[x.name_index as usize - 1].utf8().to_string()),
            ConstantPoolItem::Utf8(x) => Self::String(x.to_string()),
            ConstantPoolItem::Methodref(x) => Self::Method(ReferenceConstant::from_reference_info(constant_pool, x)),
            ConstantPoolItem::Fieldref(x) => Self::Field(ReferenceConstant::from_reference_info(constant_pool, x)),
            _ => panic!("Invalid constant pool item {:?}", constant_pool[index]),
        }
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ReferenceConstant {
    pub class: String,
    pub name: String,
    pub descriptor: String,
}

impl ReferenceConstant {
    pub fn from_constant_pool(constant_pool: &[ConstantPoolItem], index: usize) -> Self {
        match &constant_pool[index - 1] {
            ConstantPoolItem::Fieldref(x) => Self::from_reference_info(constant_pool, x),
            ConstantPoolItem::Methodref(x) => Self::from_reference_info(constant_pool, x),
            _ => panic!("Invalid constant pool item {:?}", constant_pool[index]),
        }
    }

    pub fn from_reference_info(constant_pool: &[ConstantPoolItem], reference_info: &ConstantReferenceInfo) -> Self {
        let class = constant_pool[reference_info.class_index as usize - 1].class();
        let class_name = constant_pool[class.name_index as usize - 1].utf8().to_string();

        let name_and_type = constant_pool[reference_info.name_and_type_index as usize - 1].name_and_type();
        let name = constant_pool[name_and_type.name_index as usize - 1].utf8().to_string();
        let descriptor = constant_pool[name_and_type.descriptor_index as usize - 1].utf8().to_string();

        Self {
            class: class_name,
            name,
            descriptor,
        }
    }
}
