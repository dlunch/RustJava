use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use core::cell::RefCell;

use jvm::{ArrayClass, ArrayClassInstance, Class, JavaType, JavaValue, JvmResult};

use crate::array_class::ArrayClassImpl;

#[derive(Debug)]
struct ArrayClassInstanceInner {
    class: Box<dyn Class>,
    length: usize,
    elements: RefCell<Vec<JavaValue>>,
}

#[derive(Debug, Clone)]
pub struct ArrayClassInstanceImpl {
    inner: Rc<ArrayClassInstanceInner>,
}

impl ArrayClassInstanceImpl {
    pub fn new(class: &ArrayClassImpl, length: usize) -> Self {
        let element_type = class.element_type_name();
        let default_value = JavaType::parse(&element_type).default();

        Self {
            inner: Rc::new(ArrayClassInstanceInner {
                class: Box::new(class.clone()),
                length,
                elements: RefCell::new(vec![default_value; length]),
            }),
        }
    }
}

impl ArrayClassInstance for ArrayClassInstanceImpl {
    fn class(&self) -> Box<dyn Class> {
        self.inner.class.clone()
    }

    fn destroy(self: Box<Self>) {}

    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> JvmResult<()> {
        anyhow::ensure!(offset + values.len() <= self.inner.length, "Array index out of bounds");

        self.inner.elements.borrow_mut().splice(offset..offset + values.len(), values.into_vec());

        Ok(())
    }

    fn load(&self, offset: usize, length: usize) -> JvmResult<Vec<JavaValue>> {
        anyhow::ensure!(offset + length <= self.inner.length, "Array index out of bounds");

        Ok(self.inner.elements.borrow()[offset..offset + length].to_vec())
    }

    fn store_bytes(&mut self, offset: usize, values: Box<[i8]>) -> JvmResult<()> {
        let values = values.into_vec().into_iter().map(JavaValue::Byte).collect::<Vec<_>>();

        self.store(offset, values.into_boxed_slice())
    }

    fn load_bytes(&self, offset: usize, length: usize) -> JvmResult<Vec<i8>> {
        let values = self.load(offset, length)?;

        Ok(values.into_iter().map(|x| x.into()).collect::<Vec<_>>())
    }

    fn length(&self) -> usize {
        self.inner.length
    }
}
