use alloc::{collections::BTreeMap, string::String, sync::Arc};

use nom::{
    bytes::complete::take,
    combinator::{flat_map, map},
    number::complete::{be_u16, u8},
    IResult,
};
use nom_derive::{NomBE, Parse};

fn parse_utf8(data: &[u8]) -> IResult<&[u8], Arc<String>> {
    map(flat_map(be_u16, take), |x: &[u8]| Arc::new(String::from_utf8(x.to_vec()).unwrap()))(data)
}

#[derive(NomBE, Debug)]
#[nom(Selector = "u8")]
pub enum ConstantPoolItem {
    #[nom(Selector = "1")]
    Utf8(#[nom(Parse = "parse_utf8")] Arc<String>),
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
    pub fn parse_all(data: &[u8]) -> IResult<&[u8], BTreeMap<u16, Self>> {
        let (remaining, count) = be_u16(data)?;

        let mut data = remaining;
        let mut result = BTreeMap::new();
        let mut i = 1;
        loop {
            let (remaining, item) = Self::parse_with_tag(data)?;
            let is_double_entry = match &item {
                Self::Long(_) | Self::Double(_) => {
                    // long or double constant takes two constant pool entries....
                    true
                }
                _ => false,
            };
            result.insert(i, item);

            data = remaining;
            i += 1;
            if is_double_entry {
                i += 1;
            }

            if i >= count {
                break;
            }
        }

        Ok((data, result))
    }

    pub fn parse_with_tag(data: &[u8]) -> IResult<&[u8], Self> {
        flat_map(u8, |x| move |i| Self::parse(i, x))(data)
    }

    pub fn utf8(&self) -> Arc<String> {
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

#[derive(Clone, Debug)]
pub enum ValueConstant {
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    String(Arc<String>),
    Class(Arc<String>),
    Method(ReferenceConstant),
    Field(ReferenceConstant),
}

impl ValueConstant {
    pub fn from_constant_pool(constant_pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> Self {
        match &constant_pool.get(&index).unwrap() {
            ConstantPoolItem::Integer(x) => Self::Integer(*x),
            ConstantPoolItem::Float(x) => Self::Float(*x),
            ConstantPoolItem::Long(x) => Self::Long(*x),
            ConstantPoolItem::Double(x) => Self::Double(*x),
            ConstantPoolItem::String { string_index } => Self::String(constant_pool.get(string_index).unwrap().utf8()),
            ConstantPoolItem::Class { name_index } => Self::Class(constant_pool.get(name_index).unwrap().utf8()),
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
            _ => panic!("Invalid constant pool item {:?}", constant_pool.get(&index).unwrap()),
        }
    }

    pub fn as_class(&self) -> &str {
        if let Self::Class(x) = self {
            x
        } else {
            panic!("Invalid constant pool item");
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReferenceConstant {
    pub class: Arc<String>,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
}

impl ReferenceConstant {
    pub fn from_constant_pool(constant_pool: &BTreeMap<u16, ConstantPoolItem>, index: u16) -> Self {
        match &constant_pool.get(&index).unwrap() {
            ConstantPoolItem::Fieldref {
                class_index,
                name_and_type_index,
            } => Self::from_reference_info(constant_pool, *class_index, *name_and_type_index),
            ConstantPoolItem::Methodref {
                class_index,
                name_and_type_index,
            } => Self::from_reference_info(constant_pool, *class_index, *name_and_type_index),
            ConstantPoolItem::InstanceMethodref {
                class_index,
                name_and_type_index,
            } => Self::from_reference_info(constant_pool, *class_index, *name_and_type_index),
            _ => panic!("Invalid constant pool item {:?}", constant_pool.get(&index).unwrap()),
        }
    }

    pub fn from_reference_info(constant_pool: &BTreeMap<u16, ConstantPoolItem>, class_index: u16, name_and_type_index: u16) -> Self {
        let class_name_index = constant_pool.get(&class_index).unwrap().class_name_index();
        let class_name = constant_pool.get(&class_name_index).unwrap().utf8();

        let (name_index, descriptor_index) = constant_pool.get(&name_and_type_index).unwrap().name_and_type();
        let name = constant_pool.get(&name_index).unwrap().utf8();
        let descriptor = constant_pool.get(&descriptor_index).unwrap().utf8();

        Self {
            class: class_name,
            name,
            descriptor,
        }
    }
}
