use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use core::cell::RefCell;

use dyn_clone::clone_box;

use jvm::{ArrayClass, ArrayClassInstance, Class, ClassInstance, JavaType, JavaValue, JvmResult};

use crate::array_class::ArrayClassImpl;

#[derive(Debug, Clone)]
pub struct ArrayClassInstanceImpl {
    class: Box<dyn Class>,
    length: usize,
    elements: Rc<RefCell<Vec<JavaValue>>>,
}

impl ArrayClassInstanceImpl {
    pub fn new(class: &ArrayClassImpl, length: usize) -> Self {
        let element_type = class.element_type_name();
        let default_value = JavaType::parse(&element_type).default();

        Self {
            class: clone_box(class),
            length,
            elements: Rc::new(RefCell::new(vec![default_value; length])),
        }
    }
}

impl ClassInstance for ArrayClassInstanceImpl {
    fn destroy(self: Box<Self>) {}

    fn class(&self) -> Box<dyn Class> {
        self.class.clone()
    }

    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        Some(self)
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        Some(self)
    }

    fn get_field(&self, _field: &dyn jvm::Field) -> JvmResult<JavaValue> {
        panic!("Array classes do not have fields")
    }

    fn put_field(&mut self, _field: &dyn jvm::Field, _value: JavaValue) -> JvmResult<()> {
        panic!("Array classes do not have fields")
    }
}

impl ArrayClassInstance for ArrayClassInstanceImpl {
    fn store(&mut self, offset: usize, values: Box<[JavaValue]>) -> JvmResult<()> {
        anyhow::ensure!(offset + values.len() <= self.length, "Array index out of bounds");

        self.elements.borrow_mut().splice(offset..offset + values.len(), values.into_vec());

        Ok(())
    }

    fn load(&self, offset: usize, length: usize) -> JvmResult<Vec<JavaValue>> {
        anyhow::ensure!(offset + length <= self.length, "Array index out of bounds");

        Ok(self.elements.borrow()[offset..offset + length].to_vec())
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
        self.length
    }
}
