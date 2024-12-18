#![no_std]
extern crate alloc;

pub mod classes;
mod loader;
mod runtime;

pub use self::{
    loader::{get_bootstrap_class_loader, get_runtime_class_proto},
    runtime::{File, FileSize, FileStat, FileType, IOError, IOResult, Runtime, SpawnCallback},
};

pub type RuntimeContext = dyn runtime::Runtime;
pub type RuntimeClassProto = java_class_proto::JavaClassProto<dyn runtime::Runtime>;

pub static RT_RUSTJAR: &str = "rt.rustjar";
