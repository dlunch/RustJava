use alloc::boxed::Box;

use crate::{Class, JvmResult};

pub trait ClassRegistry {
    fn get_class(&self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>>;
    fn register_class(&mut self, class: Box<dyn Class>);
}
