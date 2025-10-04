#![no_std]
extern crate alloc;

mod attribute;
mod class;
mod constant_pool;
mod field;
mod interface;
mod method;
mod opcode;

pub use {
    attribute::{AttributeInfo, AttributeInfoCode},
    class::ClassInfo,
    constant_pool::{ConstantPoolReference, FieldMethodref},
    field::FieldInfo,
    method::MethodInfo,
    opcode::Opcode,
};
