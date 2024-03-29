#![no_std]
extern crate alloc;

mod array_class_definition;
mod array_class_instance;
mod class_definition;
mod class_instance;
mod detail;
mod field;
mod interpreter;
mod method;
mod stack_frame;

pub use self::{
    class_definition::ClassDefinitionImpl,
    detail::JvmDetailImpl,
    field::FieldImpl,
    method::{MethodBody, MethodImpl},
};
