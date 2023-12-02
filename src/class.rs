use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

use crate::{class_definition::ClassDefinition, JavaValue, JvmResult};

pub struct Class {
    pub class_definition: ClassDefinition,
    pub storage: Vec<JavaValue>,
}

impl Class {
    pub fn new(class_definition: ClassDefinition) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            class_definition,
            storage: Vec::new(),
        }))
    }

    pub fn get_static_field(&self, field_name: &str) -> JvmResult<JavaValue> {
        self.class_definition
            .fields
            .iter()
            .find(|&field| field.name == field_name)
            .map(|field| self.storage[field.index].clone())
            .ok_or(anyhow::anyhow!("Field {} not found", field_name))
    }
}
