use alloc::{rc::Rc, string::String};

use nom::{combinator::map, number::complete::be_u16, IResult};

use crate::constant_pool::ConstantPoolItem;

pub fn parse_interface<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Rc<String>> {
    map(be_u16, |x| {
        let class = constant_pool[x as usize - 1].class();
        constant_pool[class.name_index as usize - 1].utf8()
    })(data)
}
