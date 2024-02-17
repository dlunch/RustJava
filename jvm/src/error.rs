use alloc::{
    boxed::Box,
    fmt::{self, Display, Formatter},
    string::String,
};

use crate::ClassInstance;

#[derive(Debug)]
pub enum JvmError {
    JavaException(Box<dyn ClassInstance>),
    FatalError(String),
}

impl Display for JvmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JvmError::JavaException(e) => write!(f, "Java exception: {:?}", e),
            JvmError::FatalError(e) => write!(f, "Fatal error: {}", e),
        }
    }
}

// XXX until https://github.com/rust-lang/rust/issues/103765 fixed
extern crate std;
impl std::error::Error for JvmError {
    fn description(&self) -> &str {
        match self {
            JvmError::JavaException(_) => "Java exception",
            JvmError::FatalError(_) => "Fatal error",
        }
    }
}
