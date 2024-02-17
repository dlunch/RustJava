use alloc::{collections::BTreeMap, string::String, sync::Arc};

use nom::{combinator::map, number::complete::be_u16, IResult};

use crate::constant_pool::ConstantPoolItem;

pub fn parse_interface<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
    map(be_u16, |x| {
        let class_name_index = constant_pool.get(&x).unwrap().class_name_index();
        constant_pool.get(&class_name_index).unwrap().utf8()
    })(data)
}
