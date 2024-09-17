use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
};
use core::fmt::{self, Debug, Formatter};

use jvm::{ArrayClassDefinition, ClassInstance, Result};

use crate::array_class_instance::ArrayClassInstanceImpl;

struct ArrayClassDefinitionInner {
    element_type_name: String,
}

#[derive(Clone)]
pub struct ArrayClassDefinitionImpl {
    inner: Arc<ArrayClassDefinitionInner>,
}

impl ArrayClassDefinitionImpl {
    pub fn new(element_type_name: &str) -> Self {
        Self {
            inner: Arc::new(ArrayClassDefinitionInner {
                element_type_name: element_type_name.to_string(),
            }),
        }
    }
}

impl ArrayClassDefinition for ArrayClassDefinitionImpl {
    fn element_type_name(&self) -> String {
        self.inner.element_type_name.clone()
    }

    fn instantiate_array(&self, length: usize) -> Result<Box<dyn ClassInstance>> {
        Ok(Box::new(ArrayClassInstanceImpl::new(self, length)))
    }
}

impl Debug for ArrayClassDefinitionImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayClass({})", self.inner.element_type_name)
    }
}
