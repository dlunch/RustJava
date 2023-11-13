#![no_std]
extern crate alloc;

mod class;
mod class_loader;
mod jvm;
mod method;

pub type JvmResult<T> = anyhow::Result<T>;

pub use class::Class;
pub use class_loader::ClassLoader;
pub use jvm::Jvm;
