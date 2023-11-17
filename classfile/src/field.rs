use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use nom::{combinator::map, multi::length_count, number::complete::be_u16, sequence::tuple, IResult};

use crate::{attribute::AttributeInfo, constant_pool::ConstantPoolItem};

pub struct FieldInfo {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<AttributeInfo>,
}

impl FieldInfo {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                be_u16,
                map(be_u16, |x| constant_pool[x as usize - 1].utf8()),
                map(be_u16, |x| constant_pool[x as usize - 1].utf8()),
                length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)),
            )),
            |(access_flags, name, descriptor, attributes)| Self {
                access_flags,
                name: name.to_string(),
                descriptor: descriptor.to_string(),
                attributes,
            },
        )(data)
    }
}
