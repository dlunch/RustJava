use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use nom::{
    IResult, Parser,
    combinator::{map, map_res},
    multi::length_count,
    number::complete::be_u16,
};

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
            (
                map_res(be_u16, |x| FieldAccessFlags::from_bits(x).ok_or(())),
                map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())),
                map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())),
                length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)),
            ),
            |(access_flags, name, descriptor, attributes)| Self {
                access_flags,
                name,
                descriptor,
                attributes,
            },
        )
        .parse(data)
    }
}
