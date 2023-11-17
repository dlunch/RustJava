use alloc::{rc::Rc, string::String, vec::Vec};

use nom::{combinator::map, number::complete::be_u16, IResult};
use nom_derive::{NomBE, Parse};

use crate::{attribute::AttributeInfo, constant_pool::ConstantPoolItem, field::FieldInfo, interface::parse_interface, method::MethodInfo};

fn parse_this_class<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Rc<String>> {
    map(be_u16, |x| {
        let class = constant_pool[x as usize - 1].class();
        constant_pool[class.name_index as usize - 1].utf8()
    })(data)
}

fn parse_super_class<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Option<Rc<String>>> {
    map(be_u16, |x| {
        if x != 0 {
            let class = constant_pool[x as usize - 1].class();
            let name = constant_pool[class.name_index as usize - 1].utf8();

            Some(name)
        } else {
            None
        }
    })(data)
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
