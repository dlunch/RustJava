use alloc::{collections::BTreeMap, rc::Rc, string::String, vec::Vec};

use nom::{combinator::map, multi::length_count, number::complete::be_u16, sequence::tuple, IResult};

use crate::{attribute::AttributeInfo, constant_pool::ConstantPoolItem};

bitflags::bitflags! {
    #[derive(Eq, PartialEq)]
    pub struct FieldAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const VOLATILE = 0x0040;
        const TRANSIENT = 0x0080;
        const SYNTHETIC = 0x1000;
        const ENUM = 0x4000;
    }
}

pub struct FieldInfo {
    pub access_flags: FieldAccessFlags,
    pub name: Rc<String>,
    pub descriptor: Rc<String>,
    pub attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                be_u16,
                map(be_u16, |x| constant_pool.get(&x).unwrap().utf8()),
                map(be_u16, |x| constant_pool.get(&x).unwrap().utf8()),
                length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)),
            )),
            |(access_flags, name, descriptor, attributes)| Self {
                access_flags: FieldAccessFlags::from_bits(access_flags).unwrap(),
                name,
                descriptor,
                attributes,
            },
        )(data)
    }
}
