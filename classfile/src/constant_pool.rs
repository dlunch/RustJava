use alloc::{rc::Rc, string::String};

use nom::{
    bytes::complete::take,
    combinator::{flat_map, map},
    number::complete::{be_u16, u8},
    IResult,
};
use nom_derive::{NomBE, Parse};

fn parse_utf8(data: &[u8]) -> IResult<&[u8], Rc<String>> {
    map(flat_map(be_u16, take), |x: &[u8]| Rc::new(String::from_utf8(x.to_vec()).unwrap()))(data)
}

#[derive(NomBE)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[nom(Selector = "u8")]
pub enum ConstantPoolItem {
    #[nom(Selector = "1")]
    Utf8(#[nom(Parse = "parse_utf8")] Rc<String>),
    #[nom(Selector = "3")]
    Integer(i32),
    #[nom(Selector = "4")]
    Float(f32),
    #[nom(Selector = "5")]
    Long(i64),
    #[nom(Selector = "6")]
    Double(f64),
    #[nom(Selector = "7")]
    Class { name_index: u16 },
    #[nom(Selector = "8")]
    String { string_index: u16 },
    #[nom(Selector = "9")]
    Fieldref { class_index: u16, name_and_type_index: u16 },
    #[nom(Selector = "10")]
    Methodref { class_index: u16, name_and_type_index: u16 },
    #[nom(Selector = "11")]
    InstanceMethodref { class_index: u16, name_and_type_index: u16 },
    #[nom(Selector = "12")]
    NameAndType { name_index: u16, descriptor_index: u16 },
}

impl ConstantPoolItem {
    pub fn parse_with_tag(data: &[u8]) -> IResult<&[u8], Self> {
        flat_map(u8, |x| move |i| Self::parse(i, x))(data)
    }

    pub fn utf8(&self) -> Rc<String> {
        if let ConstantPoolItem::Utf8(x) = self {
            x.clone()
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn class_name_index(&self) -> u16 {
        if let ConstantPoolItem::Class { name_index } = self {
            *name_index
        } else {
            panic!("Invalid constant pool item");
        }
    }

    pub fn name_and_type(&self) -> (u16, u16) {
        if let ConstantPoolItem::NameAndType {
            name_index,
            descriptor_index,
        } = self
        {
            (*name_index, *descriptor_index)
        } else {
            panic!("Invalid constant pool item");
        }
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum ValueConstant {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(Rc<String>),
    Class(Rc<String>),
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
            ConstantPoolItem::String { string_index } => Self::String(constant_pool[*string_index as usize - 1].utf8()),
            ConstantPoolItem::Class { name_index } => Self::Class(constant_pool[*name_index as usize - 1].utf8()),
            ConstantPoolItem::Utf8(x) => Self::String(x.clone()),
            ConstantPoolItem::Methodref {
                class_index,
                name_and_type_index,
            } => Self::Method(ReferenceConstant::from_reference_info(
                constant_pool,
                *class_index as _,
                *name_and_type_index as _,
            )),
            ConstantPoolItem::Fieldref {
                class_index,
                name_and_type_index,
            } => Self::Field(ReferenceConstant::from_reference_info(
                constant_pool,
                *class_index as _,
                *name_and_type_index as _,
            )),
            _ => panic!("Invalid constant pool item {:?}", constant_pool[index]),
        }
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ReferenceConstant {
    pub class: Rc<String>,
    pub name: Rc<String>,
    pub descriptor: Rc<String>,
}

impl ReferenceConstant {
    pub fn from_constant_pool(constant_pool: &[ConstantPoolItem], index: usize) -> Self {
        match &constant_pool[index - 1] {
            ConstantPoolItem::Fieldref {
                class_index,
                name_and_type_index,
            } => Self::from_reference_info(constant_pool, *class_index as _, *name_and_type_index as _),
            ConstantPoolItem::Methodref {
                class_index,
                name_and_type_index,
            } => Self::from_reference_info(constant_pool, *class_index as _, *name_and_type_index as _),
            _ => panic!("Invalid constant pool item {:?}", constant_pool[index]),
        }
    }

    pub fn from_reference_info(constant_pool: &[ConstantPoolItem], class_index: usize, name_and_type_index: usize) -> Self {
        let class_name_index = constant_pool[class_index - 1].class_name_index();
        let class_name = constant_pool[class_name_index as usize - 1].utf8();

        let (name_index, descriptor_index) = constant_pool[name_and_type_index - 1].name_and_type();
        let name = constant_pool[name_index as usize - 1].utf8();
        let descriptor = constant_pool[descriptor_index as usize - 1].utf8();

        Self {
            class: class_name,
            name,
            descriptor,
        }
    }
}
