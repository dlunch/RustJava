#![no_std]
extern crate alloc;

mod java_string;
mod run_class;
mod test_jvm;

pub use self::{java_string::JavaLangString, run_class::run_class, test_jvm::test_jvm};
