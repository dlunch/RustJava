#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use core::str;

use nom::{
    combinator::map,
    number::complete::{be_u16, be_u32},
};
use nom_derive::{NomBE, Parse};

pub type ClassFileResult<T> = anyhow::Result<T>;

#[derive(NomBE)]
#[nom(Exact)]
#[nom(Complete)]
pub struct ClassInfo {
    #[nom(Verify = "*magic == 0xCAFEBABE")]
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    #[nom(LengthCount = "map(be_u16, |x| x - 1)")]
    pub constant_pool: Vec<ConstantPoolInfo>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    #[nom(LengthCount = "be_u16")]
    pub interfaces: Vec<u16>,
    #[nom(LengthCount = "be_u16")]
    pub fields: Vec<FieldInfo>,
    #[nom(LengthCount = "be_u16")]
    pub methods: Vec<MethodInfo>,
    #[nom(LengthCount = "be_u16")]
    pub attributes: Vec<AttributeInfo>,
}

impl ClassInfo {
    pub fn constant_utf8(&self, index: u16) -> Option<&str> {
        if let ConstantPoolValue::Utf8(x) = &self.constant_pool[index as usize - 1].value {
            Some(str::from_utf8(&x.bytes).unwrap())
        } else {
            None
        }
    }
}

#[derive(NomBE)]
pub struct ConstantUtf8Info {
    pub length: u16,
    #[nom(Count = "length")]
    pub bytes: Vec<u8>,
}

#[derive(NomBE)]
pub struct ConstantClassInfo {
    pub name_index: u16,
}

#[derive(NomBE)]
pub struct ConstantStringInfo {
    pub string_index: u16,
}

#[derive(NomBE)]
pub struct ConstantReferenceInfo {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(NomBE)]
pub struct ConstantNameAndTypeInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(NomBE)]
#[nom(Selector = "u8")]
pub enum ConstantPoolValue {
    #[nom(Selector = "1")]
    Utf8(ConstantUtf8Info),
    #[nom(Selector = "3")]
    Integer(u32),
    #[nom(Selector = "4")]
    Float(f32),
    #[nom(Selector = "5")]
    Long(u64),
    #[nom(Selector = "6")]
    Double(f64),
    #[nom(Selector = "7")]
    Class(ConstantClassInfo),
    #[nom(Selector = "8")]
    String(ConstantStringInfo),
    #[nom(Selector = "9")]
    Fieldref(ConstantReferenceInfo),
    #[nom(Selector = "10")]
    Methodref(ConstantReferenceInfo),
    #[nom(Selector = "11")]
    InstanceMethodref(ConstantReferenceInfo),
    #[nom(Selector = "12")]
    NameAndType(ConstantNameAndTypeInfo),
}

#[derive(NomBE)]
pub struct ConstantPoolInfo {
    pub tag: u8,
    #[nom(Parse = "{ |x| ConstantPoolValue::parse(x, tag) }")]
    pub value: ConstantPoolValue,
}

#[derive(NomBE)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    #[nom(LengthCount = "be_u16")]
    pub attributes: Vec<AttributeInfo>,
}

#[derive(NomBE)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    #[nom(LengthCount = "be_u16")]
    pub attributes: Vec<AttributeInfo>,
}

#[derive(NomBE)]
pub struct AttributeInfo {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
    #[nom(Count = "attribute_length")]
    pub info: Vec<u8>,
}

#[derive(NomBE)]
pub struct CodeAttributeExceptionTable {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(NomBE)]
pub struct AttributeInfoCode {
    pub max_stack: u16,
    pub max_locals: u16,
    #[nom(LengthCount = "be_u32")]
    pub code: Vec<u8>,
    #[nom(LengthCount = "be_u16")]
    pub exception_table: Vec<CodeAttributeExceptionTable>,
    #[nom(LengthCount = "be_u16")]
    pub attributes: Vec<AttributeInfo>,
}

impl AttributeInfoCode {
    pub fn parse(data: &[u8]) -> ClassFileResult<Self> {
        let result = Parse::parse(data).map(|x| x.1).map_err(|e| anyhow::anyhow!("{}", e))?;

        Ok(result)
    }
}

pub fn parse_class(file: &[u8]) -> ClassFileResult<ClassInfo> {
    let result = ClassInfo::parse(file).map(|x| x.1).map_err(|e| anyhow::anyhow!("{}", e))?;

    Ok(result)
}
