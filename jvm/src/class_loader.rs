use crate::{class_definition::ClassDefinition, JvmResult};

pub trait ClassLoader {
    fn load(&mut self, class_name: &str) -> JvmResult<Option<ClassDefinition>>;
}
