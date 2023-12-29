use alloc::{collections::BTreeMap, rc::Rc, string::String, vec::Vec};

use nom::{combinator::map, number::complete::be_u16, IResult};
use nom_derive::{NomBE, Parse};

use crate::{attribute::AttributeInfo, constant_pool::ConstantPoolItem, field::FieldInfo, interface::parse_interface, method::MethodInfo};

fn parse_this_class<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Rc<String>> {
    map(be_u16, |x| {
        let class_name_index = constant_pool.get(&x).unwrap().class_name_index();
        constant_pool.get(&class_name_index).unwrap().utf8()
    })(data)
}

fn parse_super_class<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Option<Rc<String>>> {
    map(be_u16, |x| {
        if x != 0 {
            let class_name_index = constant_pool.get(&x).unwrap().class_name_index();
            let name = constant_pool.get(&class_name_index).unwrap().utf8();

            Some(name)
        } else {
            None
        }
    })(data)
}

bitflags::bitflags! {
    #[derive(Eq, PartialEq)]
    pub struct ClassAccessFlags: u16 {
        const PUBLIC = 0x0001;
        const FINAL = 0x0010;
        const SUPER = 0x0020;
        const INTERFACE = 0x0200;
        const ABSTRACT = 0x0400;
        const SYNTHETIC = 0x1000;
        const ANNOTATION = 0x2000;
        const ENUM = 0x4000;
    }
}

impl ClassAccessFlags {
    pub fn parse_be(data: &[u8]) -> IResult<&[u8], Self> {
        map(be_u16, |x| Self::from_bits(x).unwrap())(data)
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
    #[nom(Parse = "ConstantPoolItem::parse_all")]
    pub constant_pool: BTreeMap<u16, ConstantPoolItem>,
    pub access_flags: ClassAccessFlags,
    #[nom(Parse = "{ |x| parse_this_class(x, &constant_pool) }")]
    pub this_class: Rc<String>,
    #[nom(Parse = "{ |x| parse_super_class(x, &constant_pool) }")]
    pub super_class: Option<Rc<String>>,
    #[nom(LengthCount = "be_u16", Parse = "{ |x| parse_interface(x, &constant_pool) }")]
    pub interfaces: Vec<Rc<String>>,
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
