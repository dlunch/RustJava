use alloc::{collections::BTreeMap, string::String, sync::Arc};

use nom::{IResult, Parser, combinator::map_res, number::complete::be_u16};

use crate::constant_pool::ConstantPoolItem;

pub fn parse_interface<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
    map_res(be_u16, |x| {
        let class_name_index = constant_pool.get(&x).and_then(ConstantPoolItem::class_name_index).ok_or(())?;
        constant_pool.get(&class_name_index).and_then(ConstantPoolItem::utf8).ok_or(())
    })
    .parse(data)
}
