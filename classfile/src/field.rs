use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use nom::{combinator::map, multi::length_count, number::complete::be_u16, sequence::tuple, IResult};

use java_constants::FieldAccessFlags;

use crate::{attribute::AttributeInfo, constant_pool::ConstantPoolItem};

pub struct FieldInfo {
    pub access_flags: FieldAccessFlags,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
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
