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

impl From<JavaError> for anyhow::Error {
    fn from(e: JavaError) -> Self {
        anyhow::anyhow!("{:?}", e)
    }
}
