use alloc::boxed::Box;

use crate::{ArrayClass, Class, JvmResult};

pub trait ClassLoader {
    fn load(&mut self, class_name: &str) -> JvmResult<Option<Box<dyn Class>>>;
    fn load_array_class(&mut self, element_type_name: &str) -> JvmResult<Option<Box<dyn ArrayClass>>>;
}
