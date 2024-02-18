use alloc::{
    boxed::Box,
    fmt::{self, Display, Formatter},
    string::String,
};

use crate::ClassInstance;

#[derive(Debug)]
pub enum JavaError {
    JavaException(Box<dyn ClassInstance>),
    FatalError(String),
}

impl Display for JavaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JavaError::JavaException(e) => write!(f, "Java exception: {:?}", e),
            JavaError::FatalError(e) => write!(f, "Fatal error: {}", e),
        }
    }
}

// XXX until https://github.com/rust-lang/rust/issues/103765 fixed
extern crate std;
impl std::error::Error for JavaError {
    fn description(&self) -> &str {
        match self {
            JavaError::JavaException(_) => "Java exception",
            JavaError::FatalError(_) => "Fatal error",
        }
    }
}
