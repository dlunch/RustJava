use alloc::{boxed::Box, string::String};

use crate::ClassInstance;

#[derive(Debug)]
pub enum JvmError {
    JavaException(Box<dyn ClassInstance>),
    FatalError(String),
}
