#![no_std]
extern crate alloc;

use alloc::vec::Vec;

use nom::{combinator::map, number::complete::be_u16};
use nom_derive::{NomBE, Parse};

type ClassFileResult<T> = anyhow::Result<T>;

#[derive(NomBE)]
#[nom(Exact)]
#[nom(Complete)]
pub struct ClassFile {
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

pub fn parse_class(file: &[u8]) -> ClassFileResult<ClassFile> {
    let result = ClassFile::parse(file).map(|x| x.1).map_err(|e| anyhow::anyhow!("{}", e))?;

    Ok(result)
}
