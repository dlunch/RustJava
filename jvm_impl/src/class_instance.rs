use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use jvm::{ArrayClassInstance, Class, ClassInstance, JavaValue};

use crate::class::ClassImpl;

pub struct ClassInstanceImpl {
    class_name: String,
    _storage: Vec<JavaValue>,
}

impl ClassInstanceImpl {
    pub fn new(class: &ClassImpl) -> Self {
        let storage = class.fields().iter().filter(|x| !x.is_static()).map(|x| x.r#type().default()).collect();

        Self {
            class_name: class.name().to_string(),
            _storage: storage,
        }
    }
}

impl ClassInstance for ClassInstanceImpl {
    fn class_name(&self) -> &str {
        &self.class_name
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        None
    }
}
