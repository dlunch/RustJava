use alloc::{rc::Rc, vec::Vec};
use core::cell::RefCell;

use crate::{class_definition::ClassDefinition, JavaValue, JvmResult};

pub struct Class {
    pub class_definition: ClassDefinition,
    pub storage: Vec<JavaValue>,
}

impl Class {
    pub fn new(class_definition: ClassDefinition) -> Rc<RefCell<Self>> {
        let storage = class_definition
            .fields
            .iter()
            .filter(|x| x.is_static)
            .map(|x| x.r#type().default())
            .collect();

        Rc::new(RefCell::new(Self { class_definition, storage }))
    }

    pub fn get_static_field(&self, field_name: &str, descriptor: &str) -> JvmResult<JavaValue> {
        let field = self.class_definition.field(field_name, descriptor, true).unwrap();

        Ok(self.storage[field.index].clone())
    }
}
