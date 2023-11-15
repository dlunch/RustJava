use alloc::string::{String, ToString};

use nom::{number::complete::be_u16, IResult};

use crate::constant_pool::ConstantPoolItem;

pub fn parse_interface<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], String> {
    let (remaining, index) = be_u16(data)?;

    let class = constant_pool[index as usize - 1].class();
    let name = constant_pool[class.name_index as usize - 1].utf8().to_string();

    Ok((remaining, name))
}
