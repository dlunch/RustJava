use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};

use nom::{
    IResult, Parser,
    bytes::complete::take,
    combinator::{flat_map, map, map_res},
    multi::length_count,
    number::complete::{be_u16, be_u32},
};

use crate::{ConstantPoolReference, constant_pool::ConstantPoolItem, opcode::Opcode};

pub struct CodeAttributeExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: Option<Arc<String>>,
}

impl CodeAttributeExceptionTable {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map_res((be_u16, be_u16, be_u16, be_u16), |(start_pc, end_pc, handler_pc, catch_type)| {
            let catch_type = if catch_type != 0 {
                let index = constant_pool.get(&catch_type).and_then(ConstantPoolItem::class_name_index).ok_or(())?;
                Some(constant_pool.get(&index).and_then(ConstantPoolItem::utf8).ok_or(())?)
            } else {
                None
            };

            Ok::<_, ()>(Self {
                start_pc,
                end_pc,
                handler_pc,
                catch_type,
            })
        })
        .parse(data)
    }
}

pub struct AttributeInfoCode {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: BTreeMap<u32, Opcode>, // TODO we can store it Vec<u8> and create code iterator..
    pub exception_table: Vec<CodeAttributeExceptionTable>,
    pub attributes: Vec<AttributeInfo>,
}

impl AttributeInfoCode {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map(
            (
                be_u16,
                be_u16,
                map_res(flat_map(be_u32, take), |x: &[u8]| Self::parse_code(x, constant_pool)),
                length_count(be_u16, |x| CodeAttributeExceptionTable::parse(x, constant_pool)),
                length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)),
            ),
            |(max_stack, max_locals, code, exception_table, attributes)| Self {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            },
        )
        .parse(data)
    }

    fn parse_code(code: &[u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> Result<BTreeMap<u32, Opcode>, ()> {
        let mut result = BTreeMap::new();

        let mut data = code;
        while !data.is_empty() {
            let offset = unsafe { data.as_ptr().offset_from(code.as_ptr()) } as usize;
            let (remaining, opcode) = Opcode::parse(data, offset, constant_pool).map_err(|_| ())?;
            if remaining.len() >= data.len() {
                return Err(());
            }
            result.insert(offset as _, opcode);
            data = remaining;
        }

        Ok(result)
    }
}

pub struct AttributeInfoLineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

impl AttributeInfoLineNumberTableEntry {
    pub fn parse(data: &[u8]) -> IResult<&[u8], Self> {
        let (data, start_pc) = be_u16(data)?;
        let (data, line_number) = be_u16(data)?;

        Ok((data, Self { start_pc, line_number }))
    }
}

pub struct LocalVariableTableEntry {
    pub start_pc: u16,
    pub length: u16,
    pub name: Arc<String>,
    pub descriptor: Arc<String>,
    pub index: u16,
}

impl LocalVariableTableEntry {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map(
            (
                be_u16,
                be_u16,
                map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())),
                map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())),
                be_u16,
            ),
            |(start_pc, length, name, descriptor, index)| Self {
                start_pc,
                length,
                name,
                descriptor,
                index,
            },
        )
        .parse(data)
    }
}

pub enum AttributeInfo {
    ConstantValue(ConstantPoolReference),
    Code(AttributeInfoCode),
    StackMap(Vec<u8>),      // TODO Older variant of StackMapTable
    StackMapTable(Vec<u8>), // TODO
    Exceptions(Vec<u8>),    // TODO
    InnerClasses(Vec<u8>),  // TODO
    Synthetic(Vec<u8>),     // TODO
    SourceFile(Arc<String>),
    SourceDebugExtension,
    LineNumberTable(Vec<AttributeInfoLineNumberTableEntry>),
    LocalVariableTable(Vec<LocalVariableTableEntry>),
    BootstrapMethods(Vec<u8>), // TODO
    MethodParameters(Vec<u8>), // TODO
    NestMembers(Vec<u8>),      // TODO
    NestHost(Vec<u8>),         // TODO
    Unknown(Arc<String>, Vec<u8>),
}

impl AttributeInfo {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Self> {
        map_res(
            (
                map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())),
                flat_map(be_u32, take),
            ),
            |(name, info): (_, &[u8])| {
                Ok::<_, nom::Err<_>>(match name.as_str() {
                    "ConstantValue" => AttributeInfo::ConstantValue(Self::parse_constant_value(info, constant_pool)?.1),
                    "Code" => AttributeInfo::Code(AttributeInfoCode::parse(info, constant_pool)?.1),
                    "LineNumberTable" => {
                        AttributeInfo::LineNumberTable(length_count(be_u16, AttributeInfoLineNumberTableEntry::parse).parse(info)?.1)
                    }
                    "SourceFile" => AttributeInfo::SourceFile(Self::parse_source_file(info, constant_pool)?.1),
                    "LocalVariableTable" => AttributeInfo::LocalVariableTable(Self::parse_local_variable_table(info, constant_pool)?.1),
                    "StackMap" => AttributeInfo::StackMap(info.to_vec()),
                    "StackMapTable" => AttributeInfo::StackMapTable(info.to_vec()),
                    "Exceptions" => AttributeInfo::Exceptions(info.to_vec()),
                    "InnerClasses" => AttributeInfo::InnerClasses(info.to_vec()),
                    "Synthetic" => AttributeInfo::Synthetic(info.to_vec()),
                    "BootstrapMethods" => AttributeInfo::BootstrapMethods(info.to_vec()),
                    "MethodParameters" => AttributeInfo::MethodParameters(info.to_vec()),
                    "NestMembers" => AttributeInfo::NestMembers(info.to_vec()),
                    "NestHost" => AttributeInfo::NestHost(info.to_vec()),
                    // unrecognized attributes must be silently ignored (JVMS 4.7.1)
                    _ => AttributeInfo::Unknown(name.clone(), info.to_vec()),
                })
            },
        )
        .parse(data)
    }

    fn parse_source_file<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], Arc<String>> {
        map_res(be_u16, |x| constant_pool.get(&x).and_then(ConstantPoolItem::utf8).ok_or(())).parse(data)
    }

    fn parse_constant_value<'a>(data: &'a [u8], constant_pool: &BTreeMap<u16, ConstantPoolItem>) -> IResult<&'a [u8], ConstantPoolReference> {
        map_res(be_u16, |x| ConstantPoolReference::from_constant_pool(constant_pool, x).ok_or(())).parse(data)
    }

    fn parse_local_variable_table<'a>(
        data: &'a [u8],
        constant_pool: &BTreeMap<u16, ConstantPoolItem>,
    ) -> IResult<&'a [u8], Vec<LocalVariableTableEntry>> {
        length_count(be_u16, |x| LocalVariableTableEntry::parse(x, constant_pool)).parse(data)
    }
}
