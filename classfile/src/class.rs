use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use nom::{
    IResult, Parser,
    error::{Error, ErrorKind},
    multi::length_count,
    number::complete::{be_u16, be_u32},
};

use java_constants::ClassAccessFlags;

use crate::{
    ClassFileError, attribute::AttributeInfo, constant_pool::ConstantPoolItem, field::FieldInfo, interface::parse_interface, method::MethodInfo,
};

fn parse_this_class<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
    let (data, this_class) = be_u16(data)?;
    let class_name_index = constant_pool
        .get(&this_class)
        .and_then(ConstantPoolItem::class_name_index)
        .ok_or_else(|| nom::Err::Error(Error::new(data, ErrorKind::Verify)))?;
    let class_name = constant_pool
        .get(&class_name_index)
        .and_then(ConstantPoolItem::utf8)
        .ok_or_else(|| nom::Err::Error(Error::new(data, ErrorKind::Verify)))?;

    Ok((data, class_name))
}

fn parse_super_class<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Option<Arc<String>>> {
    let (data, super_class) = be_u16(data)?;

    let super_class = if super_class != 0 {
        let class_name_index = constant_pool
            .get(&super_class)
            .and_then(ConstantPoolItem::class_name_index)
            .ok_or_else(|| nom::Err::Error(Error::new(data, ErrorKind::Verify)))?;
        Some(
            constant_pool
                .get(&class_name_index)
                .and_then(ConstantPoolItem::utf8)
                .ok_or_else(|| nom::Err::Error(Error::new(data, ErrorKind::Verify)))?,
        )
    } else {
        None
    };

    Ok((data, super_class))
}

pub struct ClassInfo {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: BTreeMap<u16, ConstantPoolItem>, // TODO change to Vec
    pub access_flags: ClassAccessFlags,
    pub this_class: Arc<String>,
    pub super_class: Option<Arc<String>>,
    pub interfaces: Vec<Arc<String>>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeInfo>,
}

impl ClassInfo {
    fn parse_info(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, magic) = be_u32(data)?;
        if magic != 0xCAFEBABE {
            return Err(nom::Err::Error(nom::error::Error::new(data, nom::error::ErrorKind::Verify)));
        }

        let (data, minor_version) = be_u16(data)?;
        let (data, major_version) = be_u16(data)?;
        let (data, constant_pool) = ConstantPoolItem::parse_all(data)?;
        let (data, access_flags) = be_u16(data)?;
        let (data, this_class) = parse_this_class(data, &constant_pool)?;
        let (data, super_class) = parse_super_class(data, &constant_pool)?;
        let (data, interfaces) = length_count(be_u16, |x| parse_interface(x, &constant_pool)).parse(data)?;
        let (data, fields) = length_count(be_u16, |x| FieldInfo::parse(x, &constant_pool)).parse(data)?;
        let (data, methods) = length_count(be_u16, |x| MethodInfo::parse(x, &constant_pool)).parse(data)?;
        let (data, attributes) = length_count(be_u16, |x| AttributeInfo::parse(x, &constant_pool)).parse(data)?;

        Ok((
            data,
            Self {
                magic,
                minor_version,
                major_version,
                constant_pool,
                access_flags: ClassAccessFlags::from_bits_truncate(access_flags),
                this_class,
                super_class,
                interfaces,
                fields,
                methods,
                attributes,
            },
        ))
    }

    pub fn parse(file: &[u8]) -> Result<Self, ClassFileError> {
        let (remaining, result) = Self::parse_info(file).map_err(|_| ClassFileError::InvalidFormat)?;
        if !remaining.is_empty() {
            return Err(ClassFileError::InvalidFormat);
        }
        if result.major_version < 45 {
            return Err(ClassFileError::InvalidFormat);
        }
        if result.major_version > 70 {
            return Err(ClassFileError::UnsupportedVersion(result.major_version));
        }

        Ok(result)
    }
}
