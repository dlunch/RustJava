use alloc::{
    boxed::Box,
    rc::Rc,
    string::{String, ToString},
};
use core::fmt::{self, Debug, Formatter};

use jvm::{ArrayClass, ClassInstance};

use crate::array_class_instance::ArrayClassInstanceImpl;

struct ArrayClassInner {
    element_type_name: String,
}

#[derive(Clone)]
pub struct ArrayClassImpl {
    inner: Rc<ArrayClassInner>,
}

impl ArrayClassImpl {
    pub fn new(element_type_name: &str) -> Self {
        Self {
            inner: Rc::new(ArrayClassInner {
                element_type_name: element_type_name.to_string(),
            }),
        }
    }
}

impl ArrayClass for ArrayClassImpl {
    fn element_type_name(&self) -> String {
        self.inner.element_type_name.clone()
    }

    fn instantiate_array(&self, length: usize) -> Box<dyn ClassInstance> {
        Box::new(ArrayClassInstanceImpl::new(self, length))
    }
}

impl Debug for ArrayClassImpl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayClass({})", self.inner.element_type_name)
    }
}
