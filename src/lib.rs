#![no_std]
extern crate alloc;

mod class;
mod class_loader;
mod interpreter;
mod jvm;
mod method;
mod stack_frame;
mod thread;
mod value;

pub type JvmResult<T> = anyhow::Result<T>;

pub use class::ClassDefinition;
pub use class_loader::ClassLoader;
pub use jvm::Jvm;
