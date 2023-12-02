use alloc::{rc::Rc, string::String, vec::Vec};

use nom::{combinator::map, multi::length_count, number::complete::be_u16, sequence::tuple, IResult};

use crate::{attribute::AttributeInfo, constant_pool::ConstantPoolItem};

bitflags::bitflags! {
    #[derive(Eq, PartialEq)]
    pub struct MethodAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const PRIVATE = 0x0002;
        const PROTECTED = 0x0004;
        const STATIC = 0x0008;
        const FINAL = 0x0010;
        const SYNCHRONIZED = 0x0020;
        const BRIDGE = 0x0040;
        const VARARGS = 0x0080;
        const NATIVE = 0x0100;
        const ABSTRACT = 0x0400;
        const STRICT = 0x0800;
        const SYNTHETIC = 0x1000;
    }
}

pub struct MethodInfo {
    pub access_flags: MethodAccessFlags,
    pub name: Rc<String>,
    pub descriptor: Rc<String>,
    pub attributes: Vec<AttributeInfo>,
}

impl MethodInfo {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                be_u16,
                map(be_u16, |x| constant_pool[x as usize - 1].utf8()),
                map(be_u16, |x| constant_pool[x as usize - 1].utf8()),
                length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)),
            )),
            |(access_flags, name, descriptor, attributes)| Self {
                access_flags: MethodAccessFlags::from_bits(access_flags).unwrap(),
                name,
                descriptor,
                attributes,
            },
        )(data)
    }
}
