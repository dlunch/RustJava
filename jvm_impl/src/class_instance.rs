use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use jvm::{ArrayClassInstance, Class, ClassInstance, Field, JavaValue};

use crate::{class::ClassImpl, FieldImpl};

pub struct ClassInstanceImpl {
    class_name: String,
    storage: Vec<JavaValue>,
}

impl ClassInstanceImpl {
    pub fn new(class: &ClassImpl) -> Self {
        let storage = class.fields().iter().filter(|x| !x.is_static()).map(|x| x.r#type().default()).collect();

        Self {
            class_name: class.name().to_string(),
            storage,
        }
    }
}

impl ClassInstance for ClassInstanceImpl {
    fn class_name(&self) -> String {
        self.class_name.clone()
    }

    fn get_field(&self, field: &dyn Field) -> jvm::JvmResult<JavaValue> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        Ok(self.storage[field.index()].clone())
    }

    fn put_field(&mut self, field: &dyn Field, value: JavaValue) -> jvm::JvmResult<()> {
        let field = field.as_any().downcast_ref::<FieldImpl>().unwrap();

        self.storage[field.index()] = value;

        Ok(())
    }

    fn as_array_instance(&self) -> Option<&dyn ArrayClassInstance> {
        None
    }

    fn as_array_instance_mut(&mut self) -> Option<&mut dyn ArrayClassInstance> {
        None
    }
}
