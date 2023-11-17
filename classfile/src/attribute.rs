use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use nom::{
    combinator::map,
    multi::{length_count, many0},
    number::complete::{be_u16, be_u32, u8},
    sequence::tuple,
    IResult,
};
use nom_derive::{NomBE, Parse};

use crate::{constant_pool::ConstantPoolItem, opcode::Opcode};

pub enum AttributeInfoConstant {
    Long(u64),
    Float(f32),
    Double(f64),
    Integer(u32),
    String(String),
}

impl AttributeInfoConstant {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Self> {
        map(be_u16, |x| {
            let constant = &constant_pool[x as usize - 1];

            match constant {
                ConstantPoolItem::Integer(x) => AttributeInfoConstant::Integer(*x),
                ConstantPoolItem::Float(x) => AttributeInfoConstant::Float(*x),
                ConstantPoolItem::Long(x) => AttributeInfoConstant::Long(*x),
                ConstantPoolItem::Double(x) => AttributeInfoConstant::Double(*x),
                ConstantPoolItem::String(x) => AttributeInfoConstant::String(constant_pool[x.string_index as usize - 1].utf8().to_string()),
                _ => panic!(),
            }
        })(data)
    }
}

#[derive(NomBE)]
pub struct CodeAttributeExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

pub struct AttributeInfoCode {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<Opcode>,
    pub exception_table: Vec<CodeAttributeExceptionTable>,
    pub attributes: Vec<AttributeInfo>,
}

impl AttributeInfoCode {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Self> {
        map(
            tuple((
                be_u16,
                be_u16,
                map(length_count(be_u32, u8), |x| many0(Opcode::parse_with_tag)(x.as_slice()).unwrap().1),
                length_count(be_u16, CodeAttributeExceptionTable::parse),
                length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool)),
            )),
            |(max_stack, max_locals, code, exception_table, attributes)| Self {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            },
        )(data)
    }
}

#[derive(NomBE)]
pub struct AttributeInfoLineNumberTableEntry {
    pub start_pc: u16,
    pub line_number: u16,
}

pub enum AttributeInfo {
    ConstantValue(AttributeInfoConstant),
    Code(AttributeInfoCode),
    Exceptions,
    InnerClasses,
    Synthetic,
    SourceFile(String),
    SourceDebugExtension,
    LineNumberTable(Vec<AttributeInfoLineNumberTableEntry>),
    LocalVariableTable,
}

impl AttributeInfo {
    pub fn parse<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], Self> {
        map(
            tuple((map(be_u16, |x| constant_pool[x as usize - 1].utf8()), length_count(be_u32, u8))),
            |(name, info)| match name {
                "ConstantValue" => AttributeInfo::ConstantValue(AttributeInfoConstant::parse(&info, constant_pool).unwrap().1),
                "Code" => AttributeInfo::Code(AttributeInfoCode::parse(&info, constant_pool).unwrap().1),
                "LineNumberTable" => AttributeInfo::LineNumberTable(length_count(be_u16, AttributeInfoLineNumberTableEntry::parse)(&info).unwrap().1),
                "SourceFile" => AttributeInfo::SourceFile(Self::parse_source_file(&info, constant_pool).unwrap().1),
                _ => panic!("Unknown attribute {}", name),
            },
        )(data)
    }

    fn parse_source_file<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], String> {
        map(be_u16, |x| constant_pool[x as usize - 1].utf8().to_string())(data)
    }
}
