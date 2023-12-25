use alloc::boxed::Box;

use crate::{jvm::ClassRef, Class, JvmResult};

pub trait ClassRegistry {
    fn get_class(&self, class_name: &str) -> JvmResult<Option<ClassRef>>;
    fn register_class(&mut self, class: Box<dyn Class>);
}
