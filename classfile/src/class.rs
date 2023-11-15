use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use nom::{combinator::map, number::complete::be_u16};
use nom_derive::{NomBE, Parse};

use crate::{attribute::AttributeInfo, constant_pool::ConstantPoolItem, field::FieldInfo, interface::parse_interface, method::MethodInfo};

fn parse_this_class<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> nom::IResult<&'a [u8], String> {
    let (remaining, index) = be_u16(data)?;

    let class = constant_pool[index as usize - 1].class();
    let name = constant_pool[class.name_index as usize - 1].utf8().to_string();

    Ok((remaining, name))
}

fn parse_super_class<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> nom::IResult<&'a [u8], Option<String>> {
    let (remaining, index) = be_u16(data)?;

    if index != 0 {
        let class = constant_pool[index as usize - 1].class();
        let name = constant_pool[class.name_index as usize - 1].utf8().to_string();

        Ok((remaining, Some(name)))
    } else {
        Ok((remaining, None))
    }
}

#[derive(NomBE)]
#[nom(Exact)]
#[nom(Complete)]
pub struct ClassInfo {
    #[nom(Verify = "*magic == 0xCAFEBABE")]
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    #[nom(LengthCount = "map(be_u16, |x| x - 1)", Parse = "ConstantPoolItem::parse_with_tag")]
    pub constant_pool: Vec<ConstantPoolItem>,
    pub access_flags: u16,
    #[nom(Parse = "{ |x| parse_this_class(x, &constant_pool) }")]
    pub this_class: String,
    #[nom(Parse = "{ |x| parse_super_class(x, &constant_pool) }")]
    pub super_class: Option<String>,
    #[nom(LengthCount = "be_u16", Parse = "{ |x| parse_interface(x, &constant_pool) }")]
    pub interfaces: Vec<String>,
    #[nom(LengthCount = "be_u16", Parse = "{ |x| FieldInfo::parse(x, &constant_pool) }")]
    pub fields: Vec<FieldInfo>,
    #[nom(LengthCount = "be_u16", Parse = "{ |x| MethodInfo::parse(x, &constant_pool) }")]
    pub methods: Vec<MethodInfo>,
    #[nom(LengthCount = "be_u16", Parse = "{ |x| AttributeInfo::parse(x, &constant_pool) }")]
    pub attributes: Vec<AttributeInfo>,
}

impl ClassInfo {
    pub fn parse(file: &[u8]) -> anyhow::Result<Self> {
        let result = Parse::parse(file).map_err(|e| anyhow::anyhow!("{}", e))?.1;

        Ok(result)
    }
}
