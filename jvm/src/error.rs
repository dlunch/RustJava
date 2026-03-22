use alloc::{
    boxed::Box,
    fmt::{self, Display, Formatter},
};

use crate::ClassInstance;

#[derive(Debug)]
pub enum JavaError {
    JavaException(Box<dyn ClassInstance>),
}

impl Display for JavaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            JavaError::JavaException(e) => write!(f, "Java exception: {e:?}"),
        }
    }
}

impl From<JavaError> for anyhow::Error {
    fn from(e: JavaError) -> Self {
        anyhow::anyhow!("{:?}", e)
    }
}
