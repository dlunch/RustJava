use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use nom::{
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
        let (remaining, index) = be_u16(data)?;

        let constant = &constant_pool[index as usize - 1];

        let result = match constant {
            ConstantPoolItem::Integer(x) => AttributeInfoConstant::Integer(*x),
            ConstantPoolItem::Float(x) => AttributeInfoConstant::Float(*x),
            ConstantPoolItem::Long(x) => AttributeInfoConstant::Long(*x),
            ConstantPoolItem::Double(x) => AttributeInfoConstant::Double(*x),
            ConstantPoolItem::String(x) => AttributeInfoConstant::String(constant_pool[x.string_index as usize - 1].utf8().to_string()),
            _ => panic!(),
        };

        Ok((remaining, result))
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
        let (remaining, (max_stack, max_locals, code, exception_table)) = tuple((
            be_u16,
            be_u16,
            length_count(be_u32, u8),
            length_count(be_u16, CodeAttributeExceptionTable::parse),
        ))(data)?;

        let code = many0(Opcode::parse_with_tag)(code.as_slice()).unwrap().1;

        let (remaining, attributes) = length_count(be_u16, |x| AttributeInfo::parse(x, constant_pool))(remaining)?;

        Ok((
            remaining,
            Self {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            },
        ))
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
        let (remaining, name_index) = be_u16(data)?;
        let (remaining, info) = length_count(be_u32, u8)(remaining)?;

        let name = constant_pool[name_index as usize - 1].utf8();

        let result = match name {
            "ConstantValue" => AttributeInfo::ConstantValue(AttributeInfoConstant::parse(&info, constant_pool).unwrap().1),
            "Code" => AttributeInfo::Code(AttributeInfoCode::parse(&info, constant_pool).unwrap().1),
            "LineNumberTable" => AttributeInfo::LineNumberTable(length_count(be_u16, AttributeInfoLineNumberTableEntry::parse)(&info).unwrap().1),
            "SourceFile" => AttributeInfo::SourceFile(Self::parse_source_file(&info, constant_pool).unwrap().1),
            _ => panic!("Unknown attribute {}", name),
        };

        Ok((remaining, result))
    }

    fn parse_source_file<'a>(data: &'a [u8], constant_pool: &[ConstantPoolItem]) -> IResult<&'a [u8], String> {
        let (remaining, index) = be_u16(data)?;

        let name = constant_pool[index as usize - 1].utf8();

        Ok((remaining, name.to_string()))
    }
}
